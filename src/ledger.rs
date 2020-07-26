use rand::Rng;
use rocket::response::content;
use rocket::request::{Form, FormError};
use rocket::State;
use serde_json::to_string;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Display;
use std::sync::{Arc, RwLock};
use super::shop::{Vendor, Item};
use super::{nanoid, util};

pub enum LedgerError {
    ExistingVendor,
    ExistingUrl,
    InvalidVendor
}

#[derive(FromForm)]
pub struct UUID {
    pub uuid: String
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
    ledger_items: RwLock<HashSet<String>>,
    price_history: RwLock<Vec<Vec<f64>>>
}

impl Ledger {
    pub fn new() -> Ledger {
        Ledger { 
            version: 0, 
            entries: RwLock::new(vec![]),
            vendors: RwLock::new(vec![]),
            vendor_ids: RwLock::new(vec![]),
            vendor_versions: RwLock::new(vec![]),
            ledger_items: RwLock::new(HashSet::new()),
            price_history: RwLock::new(vec![vec![]; util::get_rust_types(0).len()])
        }
    }

    /// Gets the history of average prices for an item
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current ledger object
    /// * `item`    - The name of the item requested
    pub fn get_item_history(&self, item: String) -> Vec<f64> { self.price_history.read().unwrap()[util::get_rust_type_index(item)].clone() }

    /// Gets the names of all of the items currently tracked by the ledger
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current ledger object
    pub fn get_ledger_items(&self) -> Vec<String> { self.ledger_items.read().unwrap().clone().into_iter().collect::<Vec<String>>() }

    /// Gets the history of all of the items in the ledger
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current ledger object
    pub fn get_price_history(&self) -> Vec<Vec<f64>> { self.price_history.read().unwrap().clone() }

    /// Returns a copy of the vendor at the given index
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current ledger object
    /// * `index`   - The index of the vendor in the internal session list
    pub fn get_vendor(&self, index: usize) -> Vendor { self.vendors.read().unwrap()[index].clone() }

    /// Returns a copy of the vendors in the ledger
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current ledger object
    pub fn get_vendors(&self) -> Vec<Vendor> { self.vendors.read().unwrap().clone() }

    /// Returns a list containing the names of all of the vendors
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current ledger object
    pub fn get_vendor_names(&self) -> Vec<String> {
        let mut retval: Vec<String> = vec![];
        for v in self.vendors.read().unwrap().iter() {
            retval.push(v.name.clone());
        }
        retval
    }

    /// Returns a list containing the urls of all of the vendors
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current ledger object
    pub fn get_vendor_urls(&self) -> Vec<String> {
        let mut retval: Vec<String> = vec![];
        for v in self.vendors.read().unwrap().iter() {
            retval.push(v.url.clone());
        }
        retval
    }
    
    /// Returns the current latest version of the ledger
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current ledger object
    pub fn get_version(&self) -> u32 { self.version }

    /// Performs a purchase transaction where the buyer purchases stocked items
    /// from the seller for a fixed price. Confirmed purchases are final and
    /// recorded in the ledger
    /// 
    /// # Arguments
    /// 
    /// * `self`        - A mutable reference to the current ledger object
    /// * `order`       - The purchase order for the transaction, buyer and seller
    ///                   already confirmed
    /// * `seller_pos`  - The location of the seller in the internal vendor list
    /// * `buyer_pos`   - The location of the buyer in the internal vendor list
    /// * `item_price`  - The price of the item in the transaction
    pub fn purchase(&mut self, order: super::purchase::Order, seller_pos: usize, buyer_pos: usize, item_price: f64) -> u32 {
        let understock: u32;
        {
            let mut mut_vendors = self.vendors.write().unwrap();
            let mut entries = self.entries.write().unwrap();
            understock = match mut_vendors[seller_pos].purchase_item(&order.item, order.count) { Ok(u) => u, Err(_) => 0 };
            let sold = order.count - understock;
            entries.push(Entry::new(self.version + 1, mut_vendors[seller_pos].name.clone(), order.item.clone(), -1 * sold as i32, sold as f64 * item_price));

            mut_vendors[buyer_pos].add_item(Item::new(order.item.clone(), item_price, order.count, 0), false);
            mut_vendors[buyer_pos].bits -= sold as f64 * item_price;
            entries.push(Entry::new(self.version + 2, mut_vendors[buyer_pos].name.clone(), order.item.clone(), sold as i32, -1.0 * sold as f64 * item_price));
        }

        self.version += 2;
        self.update_avg_price(util::convert_minimal_to_full(self.calculate_avg_prices()));
        understock
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

        let initial_bits = match super::get_config::<f64>("initial_bits") { Some(ib) => ib, None => 1000.0 };
        let item_count = match super::get_config::<u32>("item_count") { Some(ic) => ic, None => 50 };
        let min_items = match super::get_config::<usize>("min_items") { Some(mai) => mai, None => 3 };
        let max_items = match super::get_config::<usize>("max_items") { Some(mii) => mii, None => 6 };

        let mut retval = Vendor::new(name.clone(), url.unwrap(), initial_bits);
        {
            let mut entries = self.entries.write().unwrap();
            let mut rng = rand::thread_rng();

            for t in util::get_rust_types(rng.gen_range(min_items, max_items + 1)).iter() {
                {
                    self.ledger_items.write().unwrap().insert(t.to_string());
                }
                let i1 = Item::new(t.to_string(), 0.0, 0, item_count);
                entries.push(Entry::new(self.version + 1, retval.name.clone(), i1.name.clone(), i1.get_count() as i32, i1.price));
                &retval.add_item(i1, false);
            }
        }

        self.version += 4;

        self.update_avg_price(util::convert_minimal_to_full(self.calculate_avg_prices()));

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

    /// Serializes the ledger state into a mapping from vendor names to their
    /// list of items with parallel lists for price and stock of that item
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current ledger object
    pub fn serialize_state(&self) -> HashMap<String, (Vec<String>, Vec<f64>, Vec<u32>)> {
        let mut retval = HashMap::new();
        for vendor in self.vendors.read().unwrap().iter() {
            let mut item_names = vec![];
            let mut item_prices = vec![];
            let mut item_stock = vec![];
            for item in vendor.get_items().iter() {
                item_names.push(item.name.clone());
                item_prices.push(item.price);
                item_stock.push(item.get_count());
            }
            retval.insert(vendor.name.clone(), (item_names, item_prices, item_stock));
        }
        retval
    }

    /// Serializes a vendor using the same list structure in the ledger state
    /// 
    /// # Arguments
    /// 
    /// * `self`        - The current ledger object
    /// * `vendor_id`   - The vendor to serialize
    pub fn serialize_vendor(&self, vendor_id: usize) -> (Vec<String>, Vec<f64>, Vec<u32>) {
        let mut item_names = vec![];
        let mut item_store = vec![];
        for item in self.get_vendor(vendor_id).get_items().iter() {
            item_names.push(item.name.clone());
            item_store.push(item.get_stored());
        }
        (item_names, vec![], item_store)
    }

    /// Prints the current average prices for all items in the ledger
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current ledger object
    pub fn show_avg_prices(&self) { println!("{:#?}", self.calculate_avg_prices()) }

    /// Updates a single item in the ledger
    /// 
    /// # Arguments
    /// 
    /// * `vendor_id`   - The ID of the vendor whose item needs changing
    /// * `item`        - The item to change
    /// * `price`       - New price of the item
    /// * `count`       - The change from store to stock
    pub fn update_item(&mut self, vendor_id: usize, item: String, price: f64, count: i32) {
        self.vendors.write().unwrap()[vendor_id].update_item(item, price, count);
    }

    /// Verifies the nanoid of a user request and returns internal vendor list
    /// index if it exists
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current ledger object
    /// * `uuid`    - A unique user ID for the current session
    pub fn verify_uuid(&self, uuid: String) -> Result<usize, LedgerError> {
        match self.vendor_ids.read().unwrap().iter().position(|x| x == &uuid) {
            Some(u) => Ok(u),
            None => Err(LedgerError::InvalidVendor)
        }
    }

    fn calculate_avg_prices(&self) -> HashMap<String, f64> {
        let mut mapping = HashMap::new();
        let mut reverse = HashMap::new();
        let ledger_items = self.ledger_items.read().unwrap();
        for (i, v) in ledger_items.iter().enumerate(){
            mapping.insert(v, i);
            reverse.insert(i, v);
        }
        let mut totals = vec![0.0; ledger_items.len()];
        let mut counts = totals.clone();
        let mut retval: HashMap<String, f64> = HashMap::new();

        for id in 0..self.vendor_ids.read().unwrap().len() {
            for i in self.get_vendor_items(id) {
                totals[mapping[&i.name]] += i.get_count() as f64 * i.price;
                counts[mapping[&i.name]] += i.get_count() as f64;
            }
        }

        for (i, t) in totals.iter().enumerate() {
            retval.insert(reverse[&i].clone(), t/(counts[i] as f64));
        }

        retval
    }

    fn get_vendor_items(&self, index: usize) -> Vec<Item> {
        let mut retval = vec![];
        for i in self.vendors.read().unwrap()[index].get_items().iter() {
            retval.push(Item::new(i.name.clone(), i.price.clone(), i.get_count(), 0));
        }
        retval
    }

    fn update_avg_price(&mut self, new_vals: Vec<f64>) {
        for (i, &v) in new_vals.iter().enumerate() {
            self.price_history.write().unwrap()[i].push(v);
        }
    }
}

pub struct MutLedger { pub session_ledger: Arc<RwLock<Ledger>> }

/// Endpoint to get ledger data via http request
/// 
/// # Arguments
/// 
/// * `uuid`    - The unique user ID of the vendor requesting the current
///               ledger state, this is to confirm legitimacy with the server
#[allow(unused_assignments, unused_variables)]
#[post("/ledger_state", data="<uuid>")]
pub fn request_ledger_state(uuid: Result<Form<UUID>, FormError<'_>>, ledger: State<MutLedger>) -> content::Json<String> {
    let arc_ledger = ledger.inner().session_ledger.clone();
    let internal_id: usize;
    let serialized_vendor;
    let mut output_vars: BTreeMap<String, Box<dyn Display>> = BTreeMap::new();
    let mut ledger_state = match uuid {
        Ok(u) => {
            let ledger = &*arc_ledger.read().unwrap();
            match ledger.verify_uuid(u.into_inner().uuid) {
                Ok(id) => {
                    internal_id = id;
                    serialized_vendor = ledger.serialize_vendor(id);
                    ledger.serialize_state()
                },
                Err(_) => {
                    output_vars.insert("UUID".to_string(), Box::new("not found".to_string()));
                    return util::construct_json(&output_vars);
                }
            }
        },
        Err(_) => {
            output_vars.insert("Form".to_string(), Box::new("incorrectly formatted".to_string()));
            return util::construct_json(&output_vars);
        }
    };

    {
        let ledger = &*arc_ledger.write().unwrap();
        ledger.vendor_versions.write().unwrap()[internal_id] = ledger.version;
    }

    ledger_state.insert("stored".to_string(), serialized_vendor);

    return content::Json(to_string(&ledger_state).unwrap());
}

/// Endpoint to get vendor names via http request
#[get("/vendor_names")]
pub fn request_vendor_names(ledger: State<MutLedger>) -> content::Json<String> {
    let arc_ledger = ledger.inner().session_ledger.clone();
    let ledger = &*arc_ledger.read().unwrap();
    let vendor_names = ledger.get_vendor_names();
    return content::Json(to_string(&vendor_names).unwrap());
}

/// Endpoint to get vendor urls via http request
#[get("/vendor_urls")]
pub fn request_vendor_urls(ledger: State<MutLedger>) -> content::Json<String> {
    let arc_ledger = ledger.inner().session_ledger.clone();
    let ledger = &*arc_ledger.read().unwrap();
    let vendor_urls = ledger.get_vendor_urls();
    return content::Json(to_string(&vendor_urls).unwrap());
}