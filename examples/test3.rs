extern crate blurz;

use std::time::Duration;
use std::thread;

use blurz::bluetooth_adapter::BluetoothAdapter as Adapter;
use blurz::bluetooth_device::BluetoothDevice as Device;
use blurz::bluetooth_discovery_session::BluetoothDiscoverySession as DiscoverySession;
use blurz::errors::*;

fn test3() -> Result<()> {
    let adapter: Adapter = Adapter::init()?;
    adapter.set_powered(true)?;
    loop {
        let session = DiscoverySession::create_session(adapter.get_id())?;
        thread::sleep(Duration::from_millis(200));
        session.start_discovery()?;
        thread::sleep(Duration::from_millis(800));
        let devices = adapter.get_device_list()?;

        println!("{} device(s) found", devices.len());
        'device_loop: for d in devices {
            let device = Device::new(d.clone());
            println!(
                "{} {:?} {:?}",
                device.get_id(),
                device.get_address(),
                device.get_rssi()
            );
            adapter.remove_device(device.get_id())?;
        }
        session.stop_discovery()?;
    }
}

fn main() {
    match test3() {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}
