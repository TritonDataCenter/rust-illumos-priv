// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Copyright 2021 Joyent, Inc.

#![deny(warnings)]
#![deny(missing_docs)]

//! illumos implements a set of privileges that provide fine-grained control over the actions of
//! processes. The possession of a certain privilege allows a process to perform a specific set of
//! restricted operations.
//!
//! This crate provides a safe wrapper around this interface and lets you add/remove/replace a
//! privilege set for a process or its off-spring.
//!
//! # Example:
//! ```
//! use illumos_priv::{PrivOp, PrivPtype, PrivSet, Privilege};
//!
//! // Get a new basic PrivSet.
//! let mut set = PrivSet::new_basic().unwrap();
//!
//! // Remove the ability to fork(2) from the set.
//! let _ = set
//!     .delset(Privilege::ProcFork)
//!     .expect("failed to delete from set");
//!
//! // Replace the effective privilege set with the new one
//! illumos_priv::setppriv(PrivOp::Set, PrivPtype::Effective, &set).unwrap();
//!
//! ```

use std::ffi::CStr;
use std::io;
use std::os::raw::c_char;

mod ffi;
mod privileges;

// Have to use "crate::" here due to a bug in rust 1.31 which is used by jenkins
pub use crate::privileges::Privilege;

/// See GETPPRIV(2) for more in depth documentation.
pub enum PrivPtype {
    /// Set of privileges currently in effect.
    Effective,
    /// Set of privileges that comes into effect on exec.
    Inheritable,
    /// Set of privileges that can be put into the effective set without restriction.
    Permitted,
    /// Set of privileges that determines the absolute upper bound of privileges this process and
    /// its off-spring can obtain.
    Limit,
}

impl PrivPtype {
    fn as_str(&self) -> &str {
        match self {
            PrivPtype::Effective => "Effective\0",
            PrivPtype::Inheritable => "Inheritable\0",
            PrivPtype::Permitted => "Permitted\0",
            PrivPtype::Limit => "Limit\0",
        }
    }

    fn as_ptr(&self) -> *const c_char {
        CStr::from_bytes_with_nul(self.as_str().as_bytes())
            .expect("all variants should be nul terminated")
            .as_ptr()
    }
}

/// See GETPPRIV(2) for more in depth documentation.
#[repr(C)]
pub enum PrivOp {
    /// Turns on the privileges in a `PrivSet`
    On = 0,
    /// Turns off the privileges in a `PrivSet`
    Off,
    /// Replaces the privileges in a `PrivSet`
    Set,
}

/// An illumos privilege set that allows one to add/remove or complete replace a set of privileges
/// for a process. When `PrivSet` is dropped, its backing memory is freed.
pub struct PrivSet {
    inner: *mut ffi::OpaquePrivSet,
}

impl PrivSet {
    /// Allocates a new empty `PrivSet`
    pub fn new_empty() -> io::Result<Self> {
        unsafe {
            let inner = ptr_or_err(ffi::priv_allocset())?;
            ffi::priv_emptyset(inner);
            Ok(PrivSet { inner })
        }
    }

    /// Allocates a new `PrivSet` with "basic" privileges.
    /// "basic" privileges are "privileges" unprivileged processes are accustomed to having.
    pub fn new_basic() -> io::Result<Self> {
        unsafe {
            let inner = ptr_or_err(ffi::priv_allocset())?;
            ffi::priv_basicset(inner);
            Ok(PrivSet { inner })
        }
    }

    /// Adds the "basic" set to the `PrivSet`.
    pub fn basic(&mut self) {
        unsafe {
            ffi::priv_basicset(self.inner);
        }
    }

    /// Empties the `PrivSet` so that it contains no "privileges"
    pub fn empty(&mut self) {
        unsafe {
            ffi::priv_emptyset(self.inner);
        }
    }

    /// Adds the named privilege to the `PrivSet`
    pub fn addset(&mut self, p: Privilege) -> io::Result<()> {
        unsafe { ret_or_err(ffi::priv_addset(self.inner, p.as_ptr())) }
    }

    /// Removes the named privilege from the `PrivSet`
    pub fn delset(&mut self, p: Privilege) -> io::Result<()> {
        unsafe { ret_or_err(ffi::priv_delset(self.inner, p.as_ptr())) }
    }

    /// Determines whether the `PrivSet` is empty.
    pub fn is_empty(&self) -> bool {
        unsafe { true_or_false(ffi::priv_isemptyset(self.inner)) }
    }

    /// Determines whether the named privilege is a member of the `PrivSet`.
    pub fn is_member(&self, p: Privilege) -> bool {
        unsafe { true_or_false(ffi::priv_ismember(self.inner, p.as_ptr())) }
    }

    /// Determines whether the `PrivSet` is equal to the `dst` `PrivSet`.
    pub fn is_equal(&self, dst: &PrivSet) -> bool {
        unsafe { true_or_false(ffi::priv_isequalset(self.inner, dst.inner)) }
    }
}

impl PartialEq for PrivSet {
    fn eq(&self, other: &Self) -> bool {
        self.is_equal(other)
    }
}

/// Sets or changes the processes privilege set.
pub fn setppriv(op: PrivOp, ptype: PrivPtype, sp: &PrivSet) -> io::Result<()> {
    unsafe { ret_or_err(ffi::setppriv(op as i32, ptype.as_ptr(), sp.inner)) }
}

/// Gets the process privilege set for the given `PrivPtype`.
pub fn getppriv(ptype: PrivPtype) -> io::Result<PrivSet> {
    unsafe {
        let inner = ptr_or_err(ffi::priv_allocset())?;
        // Make sure we create an instance of `PrivSet` first so in the event the call to getppriv
        // below fails, the inner value will be freed via `Drop`
        let sp = PrivSet { inner };
        ret_or_err(ffi::getppriv(ptype.as_ptr(), sp.inner))?;
        Ok(sp)
    }
}

impl Drop for PrivSet {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                ffi::priv_freeset(self.inner);
            }
        }
    }
}

// ============ Helpers ============

fn ptr_or_err<T>(ptr: *mut T) -> io::Result<*mut T> {
    if ptr.is_null() {
        Err(io::Error::last_os_error())
    } else {
        Ok(ptr)
    }
}

fn ret_or_err(ret: i32) -> io::Result<()> {
    match ret {
        -1 => Err(io::Error::last_os_error()),
        _ => Ok(()),
    }
}

fn true_or_false(ret: i32) -> bool {
    /*
     * Jenkins builds with rust 1.4.0, which doesn't support the matches!()
     * macro, but we also don't want to throw clippy warnings on any consumers
     * using a later rust.
     * Once issue #7 is fixed, which will allow Jenkins to use a later version
     * of rust, this can be switched back to matches!().
     * We also need to allow unknown clippy lints for older versions of rust
     * to pass make check.
     */
    #[allow(clippy::unknown_clippy_lints)]
    #[allow(clippy::match_like_matches_macro)]
    match ret {
        1 => true,
        _ => false,
    }
}

// ============ Tests ============

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn empty_set_test() {
        let set = PrivSet::new_empty().unwrap();
        assert_eq!(true, set.is_empty(), "set is empty");
    }

    #[test]
    fn empty_test() {
        let mut set = PrivSet::new_basic().unwrap();
        assert_eq!(false, set.is_empty(), "set is not empty");
        set.empty();
        assert_eq!(true, set.is_empty(), "set is empty");
    }

    #[test]
    fn is_equal_test() {
        let src = PrivSet::new_basic().unwrap();
        let mut dst = PrivSet::new_empty().unwrap();
        assert_eq!(false, src.is_equal(&dst), "PrivSets are not equal");
        dst.basic();
        assert_eq!(true, src.is_equal(&dst), "PrivSets are equal");
        // Also verify that PartialEq returns true
        assert!(src == dst, "PrivSets are equal");
    }

    #[test]
    fn is_member_test() {
        let set = PrivSet::new_basic().unwrap();
        assert_eq!(
            true,
            set.is_member(Privilege::ProcFork),
            "PRIV_PROC_FORK is in the set"
        );
    }

    #[test]
    fn add_priv_test() {
        let mut set = PrivSet::new_empty().unwrap();
        assert_eq!(
            false,
            set.is_member(Privilege::ProcFork),
            "PRIV_PROC_FORK is not in the set"
        );
        let _ = set
            .addset(Privilege::ProcFork)
            .expect("failed to add to the set");
        assert_eq!(
            true,
            set.is_member(Privilege::ProcFork),
            "PRIV_PROC_FORK is in the set"
        );
    }

    #[test]
    fn del_priv_test() {
        let mut set = PrivSet::new_basic().unwrap();
        assert_eq!(
            true,
            set.is_member(Privilege::ProcFork),
            "PRIV_PROC_FORK is in the set"
        );
        let _ = set
            .delset(Privilege::ProcFork)
            .expect("failed to delete from set");
        assert_eq!(
            false,
            set.is_member(Privilege::ProcFork),
            "PRIV_PROC_FORK is not in the set"
        );
    }

    #[test]
    fn getppriv_test() {
        let orig = getppriv(PrivPtype::Effective).unwrap();
        let mut src = PrivSet::new_basic().unwrap();
        let _ = src
            .delset(Privilege::ProcFork)
            .expect("failed to delete from set");
        setppriv(PrivOp::Set, PrivPtype::Effective, &src).unwrap();

        let dst = getppriv(PrivPtype::Effective).unwrap();
        assert!(src == dst, "getpprive PrivSet matches the one we just set");
        // Reset the original privilege set so other tests don't fail
        setppriv(PrivOp::Set, PrivPtype::Effective, &orig).unwrap();
    }

    #[test]
    fn drop_fork_test() {
        let orig = getppriv(PrivPtype::Effective).unwrap();
        let mut set = PrivSet::new_basic().unwrap();
        let _ = set
            .delset(Privilege::ProcFork)
            .expect("failed to delete from set");

        let res = Command::new("ls").output();
        assert!(res.is_ok(), "successfully ran ls");

        // Drop proc_fork and make sure the command fails below
        setppriv(PrivOp::Set, PrivPtype::Effective, &set).unwrap();

        let res = Command::new("ls").output();
        assert!(res.is_err(), "can no longer run ls");
        let err = res.unwrap_err();
        assert_eq!(
            std::io::ErrorKind::PermissionDenied,
            err.kind(),
            "got permission denied when attempting to run ls"
        );

        // Reset the original privilege set so other tests don't fail
        setppriv(PrivOp::Set, PrivPtype::Effective, &orig).unwrap();
    }
}
