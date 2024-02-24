# experiment-mmap-rs

Experiment of calling raw binary from Rust code with mmap

## Getting Started

1. Clone this repository

```console
git clone git@github.com:J3m3/experiment-mmap-rs.git <path>
cd <path>
```

2. Generate raw binary from aarch64 assembly

```console
cd example
make hello.bin
```

3. Run main.rs

```console
cd <path>
cargo run -- ./example/hello.bin
```