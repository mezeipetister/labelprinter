use gio::{resources_register, Resource};
use glib::Bytes;
use std::error::Error;

pub fn init() -> Result<(), Box<dyn Error>> {
    // load the gresource binary at build time and include/link it into the final binary.
    let res_bytes = include_bytes!("../data/resources.gresource");

    // Create Resource, it will live as long the value lives.
    let gbytes = Bytes::from(res_bytes.as_ref());
    let resource = Resource::new_from_data(&gbytes)?;

    // Register the resource so It wont be dropped and will continue to live in memory.
    resources_register(&resource);

    Ok(())
}
