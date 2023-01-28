# memweb-service

## Design decision
* **Pointers** are represented using a `u64` type instead of the more commonly used `usize`. This is because we read and write memory through syscalls that always takes an `unsigned long`, regardless if the platform is 64-bit or 32-bit.