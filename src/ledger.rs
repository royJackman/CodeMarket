use super::shop::{Vendor, Item};
use super::nanoid;
use rand::Rng;
use std::sync::{Arc, Mutex, RwLock};

pub enum LedgerError {
    ExistingVendor,
    ExistingUrl,
    InvalidVendor
}

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
    pub vendors: Mutex<Vec<Vendor>>,
    entries: Mutex<Vec<Entry>>,
    vendor_ids: Mutex<Vec<String>>,
    vendor_versions: Mutex<Vec<u32>>
}

impl Ledger {
    pub fn new() -> Ledger {
        Ledger { 
            version: 0, 
            entries: Mutex::new(vec![]),
            vendors: Mutex::new(vec![]),
            vendor_ids: Mutex::new(vec![]),
            vendor_versions: Mutex::new(vec![])
        }
    }

    pub fn register_vendor(&mut self, name: String, url: Option<String>) -> Result<String, LedgerError> {
        let mut url = url;

        {
            let market = self.vendors.lock().unwrap();
            if let Some(_) = market.iter().find(|x| x.name == name) {
                return Err(LedgerError::ExistingVendor);
            }
            if let Some(u) = &url {
                if let Some(_) = market.iter().find(|x| &x.url == u) {
                    return Err(LedgerError::ExistingUrl);
                }
            } else {
                url = Some(name.to_lowercase().replace(" ", "_"));
            }
        }

        let mut retval = Vendor::new(name.clone(), url.unwrap(), 1000.0);
        {
            let mut entries = self.entries.lock().unwrap();
            let mut rng = rand::thread_rng();

            let i1 = Item::new("f32".to_string(), rng.gen_range(4.0, 6.0), rng.gen_range(40, 60));
            entries.push(Entry::new(self.version + 1, retval.name.clone(), i1.name.clone(), i1.get_count() as i32));
            &retval.add_item(i1);

            let i1 = Item::new("str".to_string(), rng.gen_range(4.0, 6.0), rng.gen_range(40, 60));
            entries.push(Entry::new(self.version + 2, retval.name.clone(), i1.name.clone(), i1.get_count() as i32));
            &retval.add_item(i1);

            let i1 = Item::new("u16".to_string(), rng.gen_range(4.0, 6.0), rng.gen_range(40, 60));
            entries.push(Entry::new(self.version + 3, retval.name.clone(), i1.name.clone(), i1.get_count() as i32));
            &retval.add_item(i1);

            let i1 = Item::new("usize".to_string(), rng.gen_range(4.0, 6.0), rng.gen_range(40, 60));
            entries.push(Entry::new(self.version + 4, retval.name.clone(), i1.name.clone(), i1.get_count() as i32));
            &retval.add_item(i1);
        }

        self.version += 4;

        {
            self.vendors.lock().unwrap().push(retval);
        }

        {
            self.vendor_versions.lock().unwrap().push(0);
        }

        {
            let vendor_id = nanoid::simple();
            self.vendor_ids.lock().unwrap().push(vendor_id.clone());
            Ok(vendor_id)
        }
    }

    pub fn verify_uuid(&self, name: String) -> Result<usize, LedgerError> {
        match self.vendor_ids.lock().unwrap().iter().position(|x| x == &name) {
            Some(u) => Ok(u),
            None => Err(LedgerError::InvalidVendor)
        }
    }
}

pub struct MutLedger { pub session_ledger: Arc<RwLock<Ledger>> }