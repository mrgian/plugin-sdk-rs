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
    pub data: [u8; 32],
}

pub trait Common {
    fn init(&self);
    fn destroy(&self);
    fn open(&self) -> Result<(), String>;
    fn close(&self);
}

pub trait Source {
    fn next_batch(&self) -> Result<i32, String>;
}

pub trait Extract {
    fn get_fields(&self) -> &str;
    fn extract_fields(&self) -> &str;
}
