use crate::bindings::*;

#[macro_export]
macro_rules! plugin_info {
    ($name:expr, $description:expr, $contact:expr, $version:expr, $required_api_version:expr, $event_source:expr, $id:expr) => {
        const PLUGIN: Plugin = Plugin {
            name: formatcp!("{}\0", $name),
            description: formatcp!("{}\0", $description),
            contact: formatcp!("{}\0", $contact),
            version: formatcp!("{}\0", $version),
            required_api_version: formatcp!("{}\0", $required_api_version),
            event_source: formatcp!("{}\0", $event_source),
            id: $id
        };
    }
}

pub struct Plugin<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub contact: &'a str,
    pub version: &'a str,
    pub required_api_version: &'a str,
    pub event_source: &'a str,
    pub id: u32,
}

#[repr(C, packed)]
pub struct Event {
    pub ts: u64,
    pub tid: u64,
    pub len: u32,
    pub type_: u16,
    pub nparams: u32,
    pub reserved: u32,
    pub data_len: u32,
    pub plugin_id: u32,
    //pub data: [u8; 32],
}

pub trait Common<T> {
    fn init(&self) -> Box<T>;
    fn destroy(&self, state: &mut T);
}

pub trait Source<T> {
    fn open(&self, state: &mut T) -> Result<(), String>;
    fn close(&self, state: &mut T);
    fn next_batch(&self, state: &mut T) -> Result<i32, String>;
}

pub trait Extract<T> {
    fn get_fields(&self) -> &str;
    fn extract_fields(
        &self,
       state: &mut T,
        fields: &mut [ss_plugin_extract_field],
    ) -> Result<(), String>;
}
