use std::io::{Error};

pub fn start(source: String, destination: String) -> Result<(), Error> {
    println!("🚂 Building Coal Project to '{}' with source [{}]", destination, source);
    Ok(())
}