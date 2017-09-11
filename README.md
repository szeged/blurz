Bluetooth lib for Rust using blueZ/dbus
=======================================

Current state: Experimental
Required bluez version: 5.44

Examples
========
This example show how to get the first available bluetooth device.
``` rust
use blurz::{Adapter, Device};

let adapter: Adapter = Adapter::init().unwrap();
let device: Device = adapter.get_first_device().unwrap();
println!("{:?}", device);
```
