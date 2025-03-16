# Chesster
Chess engine developed in Rust.

## Building
### Requirements
- [rustc and cargo](https://www.rust-lang.org/tools/install)
- Dependencies of [raylib-rs](https://github.com/deltaphc/raylib-rs) (cmake and clang compiler mainly).

### Running
```
cargo run
```

Build without running:
```
cargo build
```

Build with optimzations:
```
cargo build --profile release-max
```
or
```
cargo run --profile release-max
```

Build docs:
```
cargo doc
```
or
```
cargo doc --open
```

## Command line arguments
- `--white_bot`: Run program with bot playing as white.
- `--black_bot`: Run program with bot playing as black.  

Both can be used at the same time.

## Controls
Click on pieces to move them. You can only click on pieces that can be currently moved.

Space to flip board.  
Left and right arrow to view move history.  
Backspace to revert game to currently viewed position.
