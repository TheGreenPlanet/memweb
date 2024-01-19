# memweb
R/W memory of any given Linux process through a websocket ASAP.

## Aim
This project aims to delve into the future of web development, specifically focusing on native web apps. By leveraging the programming language Rust and WebAssembly (WASM), the tool delivers a native-like experience in the web browser, with only a slight (~10%) performance difference.

## Design decision
* **Pointers** are represented using a `u64` type instead of the more commonly used `usize`. This is because we read and write memory through syscalls that always takes an `unsigned long`, regardless if the platform is 64-bit or 32-bit.

* **Custom Websocket Protocol** This project uses [Deku](https://github.com/sharksforarms/deku) for binary serialization and deserialization of packets sent between the service and the client. This is all done in a `big-endian` fashion. The first `u8` defines the type of the packet. See [protocol.rs](/shared/src/protocol.rs) for more info. 

## Build

Build the memweb (service) binary:
```bash
cargo build --release --package memweb
```
## Run
First navigate to the *service/* directory.
```bash
cd service/
```

Run service (in usermode)
```
cargo run
```

Run service (replacing *process_vm_writev* *process_vm_readv* with *println*)
```bash
cargo run --features "fake_read_write"
```
Install the service (temporarily)
```bash
sudo su
./install-service.sh
```
