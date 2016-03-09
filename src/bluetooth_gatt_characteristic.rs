use dbus::{Connection, BusType, Message, MessageItem};
use bluetooth_utils;

use std::error::Error;

static SERVICE_NAME: &'static str = "org.bluez";
static GATT_CHARACTERISTIC_INTERFACE: &'static str = "org.bluez.GattCharacteristic1";

#[derive(Clone, Debug)]
pub struct BluetoothGATTCharacteristic {
    object_path: String,
}

impl BluetoothGATTCharacteristic {
    pub fn new(object_path: String)
           -> BluetoothGATTCharacteristic {
        BluetoothGATTCharacteristic {
            object_path: object_path
        }
    }

    pub fn get_object_path(&self) -> String {
        self.object_path.clone()
    }

    fn get_property(&self, prop: &str) -> Result<MessageItem, Box<Error>> {
        bluetooth_utils::get_property(GATT_CHARACTERISTIC_INTERFACE, &self.object_path, prop)
    }

    fn call_method(&self, method: &str, param: Option<[MessageItem; 1]>) -> Result<(), Box<Error>> {
        bluetooth_utils::call_method(GATT_CHARACTERISTIC_INTERFACE, &self.object_path, method, param)
    }

/*
 * Properties
 */

    pub fn get_uuid(&self) -> Result<String, Box<Error>> {
        let uuid = try!(self.get_property("UUID"));
        Ok(String::from(uuid.inner::<&str>().unwrap()))
    }

    pub fn get_service(&self) -> Result<String, Box<Error>> {
        let service = try!(self.get_property("Service"));
        Ok(String::from(service.inner::<&str>().unwrap()))
    }

    pub fn get_value(&self) -> Result<Vec<u8>, Box<Error>> {
        let value = try!(self.get_property("Value"));
        let z: &[MessageItem] = value.inner().unwrap();
        let mut v: Vec<u8> = Vec::new();
        for y in z {
            v.push(y.inner::<u8>().unwrap());
        }
        Ok(v)
    }

    pub fn get_descriptors(&self) -> Result<Vec<String>, Box<Error>> {
        let descriptors = try!(self.get_property("Descriptors"));
        let z: &[MessageItem] = descriptors.inner().unwrap();
        let mut v: Vec<String> = Vec::new();
        for y in z {
            v.push(String::from(y.inner::<&str>().unwrap()));
        }
        Ok(v)
    }

    pub fn get_flags(&self) -> Result<Vec<String>, Box<Error>> {
        let flags = try!(self.get_property("Flags"));
        let z: &[MessageItem] = flags.inner().unwrap();
        let mut v: Vec<String> = Vec::new();
        for y in z {
            v.push(String::from(y.inner::<&str>().unwrap()));
        }
        Ok(v)
    }

    pub fn is_notifying(&self) -> Result<bool, Box<Error>> {
        let notifying = try!(self.get_property("Notifying"));
        Ok(notifying.inner::<bool>().unwrap())
    }

/*
 * Methods
 */

    pub fn read_value(&self) -> Result<Vec<u8>, Box<Error>> {
        let c = try!(Connection::get_private(BusType::System));
        let m = try!(Message::new_method_call(SERVICE_NAME, &self.object_path, GATT_CHARACTERISTIC_INTERFACE, "ReadValue"));
        let reply = try!(c.send_with_reply_and_block(m, 1000));
        let items: MessageItem = reply.get1().unwrap();
        let z: &[MessageItem] = items.inner().unwrap();
        let mut v: Vec<u8> = Vec::new();
        for i in z {
            v.push(i.inner::<u8>().unwrap());
        }
        Ok(v)
    }

    pub fn write_value(&self, values: Vec<u8>) -> Result<(), Box<Error>> {
        let args = {
            let mut res: Vec<MessageItem> = Vec::new();
            for v in values {
                res.push(v.into());
            }
            res
        };
        self.call_method("WriteValue", Some([MessageItem::new_array(args).unwrap()]))
    }

    pub fn start_notify(&self) -> Result<(), Box<Error>> {
        self.call_method("StartNotify", None)
    }

    pub fn stop_notify(&self) -> Result<(), Box<Error>> {
        self.call_method("StopNotify", None)
    }
}