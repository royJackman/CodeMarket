use super::shop::{Vendor, Item};
use super::nanoid;
use rand::Rng;
use rand::seq::SliceRandom;
use std::sync::{Arc, RwLock};
use std::collections::HashSet;

const RUST_TYPES: &'static [&'static str] = &["bool", "u8", "u16", "u32", "u64", "u128", "i8", "i16", "i32", "i64", "i128", "f32", "f64", "str", "char", "never"];

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
    version: u32,
    vendors: RwLock<Vec<Vendor>>,
    entries: RwLock<Vec<Entry>>,
    vendor_ids: RwLock<Vec<String>>,
    vendor_versions: RwLock<Vec<u32>>,
    ledger_items: RwLock<HashSet<String>>
}

impl Ledger {
    pub fn new() -> Ledger {
        Ledger { 
            version: 0, 
            entries: RwLock::new(vec![]),
            vendors: RwLock::new(vec![]),
            vendor_ids: RwLock::new(vec![]),
            vendor_versions: RwLock::new(vec![]),
            ledger_items: RwLock::new(HashSet::new())
        }
    }
    
    pub fn get_version(&self) -> u32 { self.version }

    pub fn get_vendor(&self, index: usize) -> Vendor {
        self.vendors.read().unwrap()[index].clone()
    }

    pub fn get_vendors(&self) -> Vec<Vendor> {
        self.vendors.read().unwrap().clone()
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

    pub fn get_vendor_items(&self, index: usize) -> Vec<Item> {
        let mut retval = vec![];
        for i in self.vendors.read().unwrap()[index].get_items().iter() {
            retval.push(Item::new(i.name.clone(), i.price.clone(), i.get_count(), 0));
        }
        retval
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

            for t in RUST_TYPES.choose_multiple(&mut rng, 4) {
                {
                    self.ledger_items.write().unwrap().insert(t.to_string());
                }
                let i1 = Item::new(t.to_string(), rng.gen_range(4.0, 6.0), rng.gen_range(40, 60), 0);
                entries.push(Entry::new(self.version + 1, retval.name.clone(), i1.name.clone(), i1.get_count() as i32, i1.price));
                &retval.add_item(i1, false);
            }
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

    pub fn purchase(&mut self, order: super::purchase::Order, seller_pos: usize, buyer_pos: usize, item_price: f64) -> u32 {
        let mut mut_vendors = self.vendors.write().unwrap();
        let mut entries = self.entries.write().unwrap();
        let understock = match mut_vendors[seller_pos].purchase_item(&order.item, order.count) { Ok(u) => u, Err(_) => 0 };
        let sold = order.count - understock;
        entries.push(Entry::new(self.version + 1, mut_vendors[seller_pos].name.clone(), order.item.clone(), -1 * sold as i32, sold as f64 * item_price));
        mut_vendors[buyer_pos].add_item(Item::new(order.item.clone(), item_price, order.count, 0), false);
        mut_vendors[buyer_pos].bits -= sold as f64 * item_price;
        entries.push(Entry::new(self.version + 2, mut_vendors[buyer_pos].name.clone(), order.item.clone(), sold as i32, -1.0 * sold as f64 * item_price));
        self.version += 2;
        understock
    }

    pub fn verify_uuid(&self, name: String) -> Result<usize, LedgerError> {
        match self.vendor_ids.read().unwrap().iter().position(|x| x == &name) {
            Some(u) => Ok(u),
            None => Err(LedgerError::InvalidVendor)
        }
    }
}

pub struct MutLedger { pub session_ledger: Arc<RwLock<Ledger>> }