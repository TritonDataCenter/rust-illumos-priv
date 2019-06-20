# rust-illumos-priv
Adjust illumos privilege sets.

illumos implements a set of privileges that provide fine-grained control over
the actions of processes. The possession of a certain privilege allows a
process to perform a specific set of restricted operations.

See PRIVILEGES(5) for a list of privileges and their descriptions, or take a
look at this crates documentation.

## Example

Dropping fork and exec privileges from a process running as root results in
failure to fork-exec `ls`. Source for the below example can be found in
[examples/fork-exec.rs](examples/fork-exec.rs).

```
root - rustdev ~/src/rust-illumos-priv (git:master) # cargo run --example fork-exec
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/examples/fork-exec`
failed to fork/exec ls: PermissionDenied
```
