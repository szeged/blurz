use dbus::MessageItem;
use bluetooth_utils;
use errors::*;

static GATT_SERVICE_INTERFACE: &'static str = "org.bluez.GattService1";

#[derive(Clone, Debug)]
pub struct BluetoothGATTService {
    object_path: String,
}

impl BluetoothGATTService {
    pub fn new(object_path: String) -> BluetoothGATTService {
        BluetoothGATTService {
            object_path: object_path,
        }
    }

    pub fn get_id(&self) -> String {
        self.object_path.clone()
    }

    fn get_property(&self, prop: &str) -> Result<MessageItem> {
        bluetooth_utils::get_property(GATT_SERVICE_INTERFACE, &self.object_path, prop)
    }

    /*
     * Properties
     */

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/gatt-api.txt#n33
    pub fn get_uuid(&self) -> Result<String> {
        let uuid = self.get_property("UUID")?;
        Ok(String::from(uuid.inner::<&str>().unwrap()))
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/gatt-api.txt#n37
    pub fn is_primary(&self) -> Result<bool> {
        let primary = self.get_property("Primary")?;
        Ok(primary.inner::<bool>().unwrap())
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/gatt-api.txt#n42
    pub fn get_device(&self) -> Result<String> {
        let device = self.get_property("Device")?;
        Ok(String::from(device.inner::<&str>().unwrap()))
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/gatt-api.txt#n48
    pub fn get_includes(&self) -> Result<Vec<String>> {
        bail!("Not implemented")
    }

    pub fn get_gatt_characteristics(&self) -> Result<Vec<String>> {
        bluetooth_utils::list_characteristics(&self.object_path)
    }
}
