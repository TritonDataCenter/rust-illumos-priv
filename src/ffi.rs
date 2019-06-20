// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Copyright 2019 Joyent, Inc.

use std::os::raw::{c_char, c_int};

#[repr(C)]
/// illumos opaque type priv_set_t
pub struct OpaquePrivSet {
    _private: [u8; 0],
}

#[no_mangle]
extern "C" {
    /// allocates sufficient memory to contain a privilege set. The value of the returned privilege
    /// set is indeterminate. The function returns NULL and sets errno when it fails to allocate
    /// memory.
    pub fn priv_allocset() -> *mut OpaquePrivSet;
    /// Frees the storage allocated by `priv_allocset()`.
    pub fn priv_freeset(sp: *mut OpaquePrivSet);
    /// Clears all privileges from sp.
    pub fn priv_emptyset(sp: *mut OpaquePrivSet);
    /// Copies the basic privilege set to sp.
    pub fn priv_basicset(sp: *mut OpaquePrivSet);
    /// Adds the named privilege priv from sp.
    pub fn priv_addset(sp: *mut OpaquePrivSet, privilege: *const c_char) -> c_int;
    /// Removes the named privilege priv from sp.
    pub fn priv_delset(sp: *mut OpaquePrivSet, privilege: *const c_char) -> c_int;
    /// checks whether the named privilege priv is a member of sp.
    pub fn priv_ismember(sp: *mut OpaquePrivSet, privilege: *const c_char) -> c_int;
    /// checks whether the sp is an empty set.
    pub fn priv_isemptyset(sp: *mut OpaquePrivSet) -> c_int;
    /// checks whether the privilege set src is equal to dst.
    pub fn priv_isequalset(src: *const OpaquePrivSet, dst: *const OpaquePrivSet) -> c_int;

    /// Sets or changes the process privilege set. The op argument specifies the operation and can
    /// be one of PRIV_OFF, PRIV_ON or PRIV_SET. The which argument specifies the name of the
    /// privilege set. The set argument specifies the set.
    pub fn setppriv(op: i32, which: *const c_char, sp: *mut OpaquePrivSet) -> c_int;
    /// Returns the process privilege set specified by which in the set pointed to by set. The
    /// memory for set is allocated with priv_allocset() and freed with priv_freeset(). Both
    /// functions are documented on the priv_addset(3C) manual page.
    pub fn getppriv(which: *const c_char, set: *mut OpaquePrivSet) -> c_int;
}
