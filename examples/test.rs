extern crate blurz;

use std::error::Error;

use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_session::BluetoothSession as Session;

fn test() -> Result<(), Box<Error + Send + Sync>> {
    let session = &Session::create_session(None).unwrap();
    let adapter: Adapter = try!(Adapter::init(session));
    let device: Device = try!(adapter.get_first_device());
    println!("{:?}", device);
    Ok(())
}

fn main() {
    match test() {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}
