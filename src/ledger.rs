use super::shop::{Vendor, Item};
use super::nanoid;
use rand::Rng;
use std::sync::{Arc, RwLock};

pub enum LedgerError {
    ExistingVendor,
    ExistingUrl,
    InvalidVendor
}

//Change of goods at a vendor
#[derive(Serialize)]
struct Entry {
    id: u32,
    vendor: String,
    attribute: String,
    change: i32,
    price: f64
}

impl Entry {
    fn new(id: u32, vendor: String, attribute: String, change: i32, price: f64) -> Entry {
        Entry { id, vendor, attribute, change, price }
    }
}

//Collection of asynchronously mutable data of transactions in the market
//Used for verifying purchases, allows for parallel reading
#[derive(Serialize)]
pub struct Ledger {
    pub version: u32,
    pub vendors: RwLock<Vec<Vendor>>,
    entries: RwLock<Vec<Entry>>,
    vendor_ids: RwLock<Vec<String>>,
    vendor_versions: RwLock<Vec<u32>>
}

impl Ledger {
    pub fn new() -> Ledger {
        Ledger { 
            version: 0, 
            entries: RwLock::new(vec![]),
            vendors: RwLock::new(vec![]),
            vendor_ids: RwLock::new(vec![]),
            vendor_versions: RwLock::new(vec![])
        }
    }

    pub fn get_vendor_names(&self) -> Vec<String> {
        let mut retval: Vec<String> = vec![];
        for v in self.vendors.read().unwrap().iter() {
            retval.push(v.name.clone());
        }
        retval
    }

    pub fn get_vendor_urls(&self) -> Vec<String> {
        let mut retval: Vec<String> = vec![];
        for v in self.vendors.read().unwrap().iter() {
            retval.push(v.url.clone());
        }
        retval
    }

    pub fn get_vendors(&self) -> Vec<Vendor> {
        self.vendors.read().unwrap().clone()
    }

    /// Creates a new vendor in the ledger, and assigns initial distribution of stocked goods
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current ledger object
    /// * `name`    - The name of the new vendor
    /// * `url`     - An optional string to use for the url
    pub fn register_vendor(&mut self, name: String, url: Option<String>) -> Result<String, LedgerError> {
        let mut url = url;

        {
            let market = self.vendors.read().unwrap();
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
            let mut entries = self.entries.write().unwrap();
            let mut rng = rand::thread_rng();

            let i1 = Item::new("f32".to_string(), rng.gen_range(4.0, 6.0), rng.gen_range(40, 60), 0);
            entries.push(Entry::new(self.version + 1, retval.name.clone(), i1.name.clone(), i1.get_count() as i32, i1.price));
            &retval.add_item(i1, false);

            let i1 = Item::new("str".to_string(), rng.gen_range(4.0, 6.0), rng.gen_range(40, 60), 0);
            entries.push(Entry::new(self.version + 2, retval.name.clone(), i1.name.clone(), i1.get_count() as i32, i1.price));
            &retval.add_item(i1, false);

            let i1 = Item::new("u16".to_string(), rng.gen_range(4.0, 6.0), rng.gen_range(40, 60), 0);
            entries.push(Entry::new(self.version + 3, retval.name.clone(), i1.name.clone(), i1.get_count() as i32, i1.price));
            &retval.add_item(i1, false);

            let i1 = Item::new("usize".to_string(), rng.gen_range(4.0, 6.0), rng.gen_range(40, 60), 0);
            entries.push(Entry::new(self.version + 4, retval.name.clone(), i1.name.clone(), i1.get_count() as i32, i1.price));
            &retval.add_item(i1, false);
        }

        self.version += 4;

        {
            self.vendors.write().unwrap().push(retval);
        }

        {
            self.vendor_versions.write().unwrap().push(0);
        }

        {
            let vendor_id = nanoid::simple();
            self.vendor_ids.write().unwrap().push(vendor_id.clone());
            Ok(vendor_id)
        }
    }

    pub fn verify_uuid(&self, name: String) -> Result<usize, LedgerError> {
        match self.vendor_ids.read().unwrap().iter().position(|x| x == &name) {
            Some(u) => Ok(u),
            None => Err(LedgerError::InvalidVendor)
        }
    }
}

pub struct MutLedger { pub session_ledger: Arc<RwLock<Ledger>> }