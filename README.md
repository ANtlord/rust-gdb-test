# rust-gdb-test
Example repository shows inability of GDB to find a symbol of third party crate.
In order to reproduce that do:
```
cargo build
rust-gdb rust-gdb target/debug/rust-gdb-test
break rust_gdb_test::main
print extern libc::SIGUSR2
```
