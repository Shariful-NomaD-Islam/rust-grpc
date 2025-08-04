# RUST REST-gRPC

### Master comunicate with outside world with REST but internal comunication between master and worker is in gRPC


### Pre-requisites:

The following sdk/packages are needed for this project

* rustup

### How to build 

```code
cargo clean
cargo build
```

### Proto file will generate in "target/debug/build/master-*/out/"

### How to run 

```code
cargo run -p master
cargo run -p worker
```


