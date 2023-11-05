#![allow(warnings)]
#![feature(pointer_byte_offsets)]
#![feature(ptr_sub_ptr)]

use hex_literal::hex;
use const_format::formatcp;
use std::ffi::*;
use std::error::Error;

use dbgtools_hexdump::{Config, hexdump};

mod bindings;
use bindings::*;

mod plugin;
use plugin::{Common, Source, Extract, Plugin, Event};

mod macros;

const dummy: Plugin = Plugin {
    name: "dummy-rs",
    description: "Falco dummy plugin written in Rust",
    contact: "Gianmatteo Palmieri <mail@gian.im>",
    version: "0.0.0",
    required_api_version: "3.0.0",
    event_source: "dummy-rs-source",
    id: 999,
};

static mut event: Event = Event {
    ts: u64::MAX,
    tid: u64::MAX,
    len: 55,
    type_: 322,
    nparams: 2,
    reserved: 4,
    data_len: 32,
    plugin_id: 999,
    data: [' ' as u8; 32],
};

static mut nevents: i32 = 0;

plugin_common!(dummy);
plugin_source!(dummy);
plugin_extract!(dummy);

impl Common for Plugin<'_> {
    fn init(&self) {
        println!("Plugin initialized");
    }

    fn destroy(&self) {
        println!("Plugin destroyed");
    }

    fn open(&self) -> Result<(), String> {
        println!("Opening stream event");

        //Err("Error while opening")
        Ok(())
    }

    fn close(&self) {
        println!("Closing stream event");
    }
}

impl Source for Plugin<'_> {
    fn next_batch(&self) -> Result<i32, String> {
        unsafe {
            nevents += 1;

            let message = "Hello Rust!";

            //write message in event payload
            for (i, c) in message.chars().enumerate() {
                event.data[i] = c as u8;
            }

            //generate 10 events
            if nevents >= 10 {
                return Ok(-1); //SS_PLUGIN_TIMEOUT
            } 
        }
        Ok(0) //SS_PLUGIN_SUCCESS
    }
}

const fields: &str = r#"
[
    {"type": "string","name": "sample.hello","desc": "An hello string"}
]
"#;
const FIELDS: &str = formatcp!("{}\0", fields);

const hello_field: &str = "Hello field!\0";

impl Extract for Plugin<'_> {
    fn get_fields(&self) -> &str {
        FIELDS
    }

    fn extract_fields(&self) -> &str{
        hello_field
    }
}
