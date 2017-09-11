use bluetooth_utils;
use dbus::MessageItem;
use hex::FromHex;
use std::collections::HashMap;
use errors::*;

static DEVICE_INTERFACE: &'static str = "org.bluez.Device1";

#[derive(Clone, Debug)]
pub struct BluetoothDevice {
    object_path: String,
}

impl BluetoothDevice {
    pub fn new(object_path: String) -> BluetoothDevice {
        BluetoothDevice {
            object_path: object_path,
        }
    }

    pub fn get_id(&self) -> String {
        self.object_path.clone()
    }

    fn get_property(&self, prop: &str) -> Result<MessageItem> {
        bluetooth_utils::get_property(DEVICE_INTERFACE, &self.object_path, prop)
    }

    fn set_property<T>(&self, prop: &str, value: T) -> Result<()>
    where
        T: Into<MessageItem>,
    {
        bluetooth_utils::set_property(DEVICE_INTERFACE, &self.object_path, prop, value)
    }

    fn call_method(&self, method: &str, param: Option<&[MessageItem]>) -> Result<()> {
        bluetooth_utils::call_method(DEVICE_INTERFACE, &self.object_path, method, param)
    }

    /*
     * Properties
     */
    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n105
    pub fn get_address(&self) -> Result<String> {
        let address = self.get_property("Address")?;
        Ok(String::from(address.inner::<&str>().unwrap()))
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n109
    pub fn get_name(&self) -> Result<String> {
        let name = self.get_property("Name")?;
        Ok(String::from(name.inner::<&str>().unwrap()))
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n121
    pub fn get_icon(&self) -> Result<String> {
        let icon = self.get_property("Icon")?;
        Ok(String::from(icon.inner::<&str>().unwrap()))
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n126
    pub fn get_class(&self) -> Result<u32> {
        let class = self.get_property("Class")?;
        Ok(class.inner::<u32>().unwrap())
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n126
    pub fn get_appearance(&self) -> Result<u16> {
        let appearance = self.get_property("Appearance")?;
        Ok(appearance.inner::<u16>().unwrap())
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n134
    pub fn get_uuids(&self) -> Result<Vec<String>> {
        let uuids = self.get_property("UUIDs")?;
        let z: &[MessageItem] = uuids.inner().unwrap();
        let mut v: Vec<String> = Vec::new();
        for y in z {
            v.push(String::from(y.inner::<&str>().unwrap()));
        }
        Ok(v)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n139
    pub fn is_paired(&self) -> Result<bool> {
        let paired = self.get_property("Paired")?;
        Ok(paired.inner::<bool>().unwrap())
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n143
    pub fn is_connected(&self) -> Result<bool> {
        let connected = self.get_property("Connected")?;
        Ok(connected.inner::<bool>().unwrap())
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n149
    pub fn is_trusted(&self) -> Result<bool> {
        let trusted = self.get_property("Trusted")?;
        Ok(trusted.inner::<bool>().unwrap())
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n154
    pub fn is_blocked(&self) -> Result<bool> {
        let blocked = self.get_property("Blocked")?;
        Ok(blocked.inner::<bool>().unwrap())
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n161
    pub fn get_alias(&self) -> Result<String> {
        let alias = self.get_property("Alias")?;
        Ok(String::from(alias.inner::<&str>().unwrap()))
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n161
    pub fn set_alias(&self, value: String) -> Result<()> {
        self.set_property("Alias", value)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n174
    pub fn get_adapter(&self) -> Result<String> {
        let adapter = self.get_property("Adapter")?;
        Ok(String::from(adapter.inner::<&str>().unwrap()))
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n178
    pub fn is_legacy_pairing(&self) -> Result<bool> {
        let legacy_pairing = self.get_property("LegacyPairing")?;
        Ok(legacy_pairing.inner::<bool>().unwrap())
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n189
    pub fn get_modalias(&self) -> Result<(String, u32, u32, u32)> {
        let modalias = self.get_property("Modalias")?;
        let m = modalias.inner::<&str>().unwrap();
        let ids: Vec<&str> = m.split(":").collect();

        let source = String::from(ids[0]);
        let vendor = Vec::from_hex(ids[1][1..5].to_string()).unwrap();
        let product = Vec::from_hex(ids[1][6..10].to_string()).unwrap();
        let device = Vec::from_hex(ids[1][11..15].to_string()).unwrap();

        Ok((
            source,
            (vendor[0] as u32) * 16 * 16 + (vendor[1] as u32),
            (product[0] as u32) * 16 * 16 + (product[1] as u32),
            (device[0] as u32) * 16 * 16 + (device[1] as u32),
        ))
    }

    pub fn get_vendor_id_source(&self) -> Result<String> {
        let (vendor_id_source, _, _, _) = self.get_modalias()?;
        Ok(vendor_id_source)
    }

    pub fn get_vendor_id(&self) -> Result<u32> {
        let (_, vendor_id, _, _) = self.get_modalias()?;
        Ok(vendor_id)
    }

    pub fn get_product_id(&self) -> Result<u32> {
        let (_, _, product_id, _) = self.get_modalias()?;
        Ok(product_id)
    }

    pub fn get_device_id(&self) -> Result<u32> {
        let (_, _, _, device_id) = self.get_modalias()?;
        Ok(device_id)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n194
    pub fn get_rssi(&self) -> Result<i16> {
        let rssi = self.get_property("RSSI")?;
        Ok(rssi.inner::<i16>().unwrap())
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n199
    pub fn get_tx_power(&self) -> Result<i16> {
        let tx_power = self.get_property("TxPower")?;
        Ok(tx_power.inner::<i16>().unwrap())
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n204
    pub fn get_manufacturer_data(&self) -> Result<HashMap<u16, Vec<u8>>> {
        let manufacturer_data_array = self.get_property("ManufacturerData")?;
        let mut m = HashMap::new();
        let dict_vec = manufacturer_data_array
            .inner::<&Vec<MessageItem>>()
            .unwrap();
        for dict in dict_vec {
            let (key, value) = dict.inner::<(&MessageItem, &MessageItem)>().unwrap();
            let v = value
                .inner::<&MessageItem>()
                .unwrap()
                .inner::<&Vec<MessageItem>>()
                .unwrap()
                .into_iter()
                .map(|b| b.inner::<u8>().unwrap_or(0))
                .collect();
            m.insert(key.inner::<u16>().unwrap(), v);
        }
        Ok(m)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n210
    pub fn get_service_data(&self) -> Result<HashMap<String, Vec<u8>>> {
        let service_data_array = self.get_property("ServiceData")?;
        let mut m = HashMap::new();
        let dict_vec = service_data_array.inner::<&Vec<MessageItem>>().unwrap();
        for dict in dict_vec {
            let (key, value) = dict.inner::<(&MessageItem, &MessageItem)>().unwrap();
            let v = value
                .inner::<&MessageItem>()
                .unwrap()
                .inner::<&Vec<MessageItem>>()
                .unwrap()
                .into_iter()
                .map(|b| b.inner::<u8>().unwrap_or(0))
                .collect();
            m.insert(key.inner::<&str>().unwrap().to_string(), v);
        }
        Ok(m)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n215
    pub fn get_gatt_services(&self) -> Result<Vec<String>> {
        bluetooth_utils::list_services(&self.object_path)
    }

    /*
     * Methods
     */

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n12
    pub fn connect(&self) -> Result<()> {
        self.call_method("Connect", None)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n29
    pub fn disconnect(&self) -> Result<()> {
        self.call_method("Disconnect", None)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n43
    pub fn connect_profile(&self, uuid: String) -> Result<()> {
        self.call_method("ConnectProfile", Some(&[uuid.into()]))
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n55
    pub fn disconnect_profile(&self, uuid: String) -> Result<()> {
        self.call_method("DisconnectProfile", Some(&[uuid.into()]))
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n70
    pub fn pair(&self) -> Result<()> {
        self.call_method("Pair", None)
    }

    // http://git.kernel.org/cgit/bluetooth/bluez.git/tree/doc/device-api.txt#n97
    pub fn cancel_pairing(&self) -> Result<()> {
        self.call_method("CancelPairing", None)
    }
}
