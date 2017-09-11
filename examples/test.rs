extern crate blurz;

use blurz::errors::*;

use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;

fn test() -> Result<()> {
    let adapter: Adapter = Adapter::init()?;
    let device: Device = adapter.get_first_device()?;
    println!("{:?}", device);
    Ok(())
}

fn main() {
    match test() {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}
