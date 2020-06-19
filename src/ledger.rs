use super::shop::Vendor;
use std::sync::Mutex;

#[derive(Serialize)]
struct Entry {
    id: u32,
    vendor: String,
    attribute: String,
    change: i32
}

impl Entry {
    fn new(id: u32, vendor: String, attribute: String, change: i32) -> Entry {
        Entry { id, vendor, attribute, change }
    }
}

#[derive(Serialize)]
pub struct Ledger {
    pub version: u32,
    entries: Mutex<Vec<Entry>>,
    vendors: Mutex<Vec<Vendor>>
}

impl Ledger {
    pub fn new() -> Ledger {
        Ledger { 
            version: 0, 
            entries: Mutex::new(vec![]),
            vendors: Mutex::new(vec![])
        }
    }
}