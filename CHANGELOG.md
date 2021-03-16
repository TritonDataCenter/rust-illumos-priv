0.2.0
=====
This release changes the function signature for functions modifying a `PrivSet`
from `&self` to `&mut self` since the backing ffi member is actually being
modified. This should convey to the consumer that mutable access is needed so
that rust can enforce the proper borrowing semantics.

* [BUG #4](https://github.com/joyent/rust-illumos-priv/pull/5):
  Fixes PrivSet should consider mutability.
