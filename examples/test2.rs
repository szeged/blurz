extern crate blurz;

static BATTERY_SERVICE_UUID: &'static str = "0000180f-0000-1000-8000-00805f9b34fb";
static COLOR_PICKER_SERVICE_UUID: &'static str = "00001812-0000-1000-8000-00805f9b34fb";

use std::time::Duration;
use std::thread;

use blurz::{Adapter, Device, GATTService, GATTCharacteristic, GATTDescriptor, DiscoverySession};
use blurz::errors::*;

fn test2() -> Result<()> {
    let adapter: Adapter = Adapter::init()?;
    let session = DiscoverySession::create_session(adapter.get_id())?;
    session.start_discovery()?;
    //let mut devices = vec!();
    for _ in 0..5 {
        let devices = adapter.get_device_list()?;
        if !devices.is_empty() {
            break;
        }
        thread::sleep(Duration::from_millis(1000));
    }
    session.stop_discovery()?;
    let devices = adapter.get_device_list()?;
    if devices.is_empty() {
        return Err(Error::from("No device found"));
    }
    println!("{} device(s) found", devices.len());
    let mut device: Device = Device::new("".to_string());
    'device_loop: for d in devices {
        device = Device::new(d.clone());
        println!("{} {:?}", device.get_id(), device.get_alias());
        let uuids = device.get_uuids()?;
        println!("{:?}", uuids);
        'uuid_loop: for uuid in uuids {
            if uuid == COLOR_PICKER_SERVICE_UUID || uuid == BATTERY_SERVICE_UUID {
                println!("{:?} has a service!", device.get_alias());
                println!("connect device...");
                device.connect().ok();
                if device.is_connected()? {
                    println!("checking gatt...");
                    // We need to wait a bit after calling connect to safely
                    // get the gatt services
                    thread::sleep(Duration::from_millis(5000));
                    match device.get_gatt_services() {
                        Ok(_) => break 'device_loop,
                        Err(e) => println!("{:?}", e),
                    }
                } else {
                    println!("could not connect");
                }
            }
        }
        println!("");
    }
    adapter.stop_discovery().ok();
    if !device.is_connected()? {
        return Err(Error::from("No connectable device found"));
    }
    let services = device.get_gatt_services()?;
    for service in services {
        let s = GATTService::new(service.clone());
        println!("{:?}", s);
        let characteristics = s.get_gatt_characteristics()?;
        for characteristic in characteristics {
            let c = GATTCharacteristic::new(characteristic.clone());
            println!("{:?}", c);
            println!("Value: {:?}", c.read_value(None));
            let descriptors = c.get_gatt_descriptors()?;
            for descriptor in descriptors {
                let d = GATTDescriptor::new(descriptor.clone());
                println!("{:?}", d);
                println!("Value: {:?}", d.read_value(None));
            }
        }
    }
    device.disconnect()?;
    Ok(())
}

fn main() {
    match test2() {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}
