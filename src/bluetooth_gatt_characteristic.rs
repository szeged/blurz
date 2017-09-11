use dbus::{BusType, Connection, Message, MessageItem};
use bluetooth_utils;

use std::borrow::Cow;

use errors::*;

static SERVICE_NAME: &'static str = "org.bluez";
static GATT_CHARACTERISTIC_INTERFACE: &'static str = "org.bluez.GattCharacteristic1";

#[derive(Clone, Debug)]
pub struct BluetoothGATTCharacteristic {
    object_path: String,
}

impl BluetoothGATTCharacteristic {
    pub fn new(object_path: String) -> BluetoothGATTCharacteristic {
        BluetoothGATTCharacteristic {
            object_path: object_path,
        }
    }

    pub fn get_id(&self) -> String {
        self.object_path.clone()
    }

    fn get_property(&self, prop: &str) -> Result<MessageItem> {
        bluetooth_utils::get_property(GATT_CHARACTERISTIC_INTERFACE, &self.object_path, prop)
    }

    fn call_method(&self, method: &str, param: Option<&[MessageItem]>) -> Result<()> {
        bluetooth_utils::call_method(
            GATT_CHARACTERISTIC_INTERFACE,
            &self.object_path,
            method,
            param,
        )
    }

    /*
     * Properties
     */

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/gatt-api.txt#n114
    pub fn get_uuid(&self) -> Result<String> {
        let uuid = self.get_property("UUID")?;
        Ok(String::from(uuid.inner::<&str>().unwrap()))
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/gatt-api.txt#n118
    pub fn get_service(&self) -> Result<String> {
        let service = self.get_property("Service")?;
        Ok(String::from(service.inner::<&str>().unwrap()))
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/gatt-api.txt#n123
    pub fn get_value(&self) -> Result<Vec<u8>> {
        let value = self.get_property("Value")?;
        let z: &[MessageItem] = value.inner().unwrap();
        let mut v: Vec<u8> = Vec::new();
        for y in z {
            v.push(y.inner::<u8>().unwrap());
        }
        Ok(v)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/gatt-api.txt#n130
    pub fn is_notifying(&self) -> Result<bool> {
        let notifying = self.get_property("Notifying")?;
        Ok(notifying.inner::<bool>().unwrap())
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/gatt-api.txt#n135
    pub fn get_flags(&self) -> Result<Vec<String>> {
        let flags = self.get_property("Flags")?;
        let z: &[MessageItem] = flags.inner().unwrap();
        let mut v: Vec<String> = Vec::new();
        for y in z {
            v.push(String::from(y.inner::<&str>().unwrap()));
        }
        Ok(v)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/gatt-api.txt#n156
    pub fn get_gatt_descriptors(&self) -> Result<Vec<String>> {
        bluetooth_utils::list_descriptors(&self.object_path)
    }

    /*
     * Methods
     */

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/gatt-api.txt#n72
    pub fn read_value(&self, offset: Option<u16>) -> Result<Vec<u8>> {
        let c = Connection::get_private(BusType::System)?;
        let mut m = Message::new_method_call(
            SERVICE_NAME,
            &self.object_path,
            GATT_CHARACTERISTIC_INTERFACE,
            "ReadValue"
        )?;
        m.append_items(&[
            MessageItem::Array(
                match offset {
                    Some(o) => vec![
                        MessageItem::DictEntry(
                            Box::new("offset".into()),
                            Box::new(MessageItem::Variant(Box::new(o.into()))),
                        ),
                    ],
                    None => vec![],
                },
                Cow::Borrowed("{sv}"),
            ),
        ]);
        let reply = c.send_with_reply_and_block(m, 1000)?;
        let items: MessageItem = reply.get1().unwrap();
        let z: &[MessageItem] = items.inner().unwrap();
        let mut v: Vec<u8> = Vec::new();
        for i in z {
            v.push(i.inner::<u8>().unwrap());
        }
        Ok(v)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/gatt-api.txt#n84
    pub fn write_value(&self, values: Vec<u8>, offset: Option<u16>) -> Result<()> {
        let values_msgs = {
            let mut res: Vec<MessageItem> = Vec::new();
            for v in values {
                res.push(v.into());
            }
            res
        };
        self.call_method(
            "WriteValue",
            Some(&[
                MessageItem::new_array(values_msgs).unwrap(),
                MessageItem::Array(
                    match offset {
                        Some(o) => vec![
                            MessageItem::DictEntry(
                                Box::new("offset".into()),
                                Box::new(MessageItem::Variant(Box::new(o.into()))),
                            ),
                        ],
                        None => vec![],
                    },
                    Cow::Borrowed("{sv}"),
                ),
            ]),
        )
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/gatt-api.txt#n96
    pub fn start_notify(&self) -> Result<()> {
        self.call_method("StartNotify", None)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/gatt-api.txt#n105
    pub fn stop_notify(&self) -> Result<()> {
        self.call_method("StopNotify", None)
    }
}
