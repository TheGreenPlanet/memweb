# memweb

## Design decision
* **Pointers** are represented using a `u64` type instead of the more commonly used `usize`. This is because we read and write memory through syscalls that always takes an `unsigned long`, regardless if the platform is 64-bit or 32-bit.

* **Custom Websocket Protocol** This project uses [Deku](https://github.com/sharksforarms/deku) to serialize and deserialize packets sent between the service and the client. This is all done in a `big-endian` fashion. The first `u8` defines the type of the packet. See [protocol.rs](/shared/src/protocol.rs) for more info. 