extern crate dbus;
extern crate hex;

#[macro_use]
extern crate error_chain;

pub use bluetooth_adapter::BluetoothAdapter as Adapter;
pub use bluetooth_device::BluetoothDevice as Device;
pub use bluetooth_gatt_characteristic::BluetoothGATTCharacteristic as GATTCharacteristic;
pub use bluetooth_gatt_descriptor::BluetoothGATTDescriptor as GATTDescriptor;
pub use bluetooth_gatt_service::BluetoothGATTService as GATTService;
pub use bluetooth_discovery_session::BluetoothDiscoverySession as DiscoverySession;

pub mod errors;
mod bluetooth_device;
mod bluetooth_adapter;
mod bluetooth_gatt_characteristic;
mod bluetooth_gatt_descriptor;
mod bluetooth_gatt_service;
mod bluetooth_discovery_session;
mod bluetooth_utils;