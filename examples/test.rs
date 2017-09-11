extern crate blurz;

use blurz::{Adapter, Device};
use blurz::errors::*;

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
