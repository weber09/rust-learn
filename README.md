# Rust Learning Repo!

Created this small repo to learn rust language.
Will be adding folders containing rust code for practice

### Running

It is used Cargo to automatically manage build artifacts.

To create a new project inside the main folder, run:

```rust
cargo new hello-world
```

To build and run the project in one command (most common during development):

```rust
cargo run
```

Or you can do it in two steps:

```rust
cargo build    # This compiles the project
./target/debug/binary-search    # This runs the compiled binary
```

By default, this will build in debug mode. When you're ready to create a release build with optimizations, you can use:

```rust
cargo build --release    # Creates an optimized build
# or
cargo run --release     # Builds and runs the optimized version
```

You can also check if your code compiles without creating an executable:

```rust
cargo check    # Faster than build, useful while developing
``` 