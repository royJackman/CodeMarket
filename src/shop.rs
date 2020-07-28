use rocket::response::content;
use rocket::request::{Form, FormError};
use rocket::State;
use rocket_contrib::templates::Template;
use super::ledger::MutLedger;
use serde_json::to_value;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Display;

pub enum ShopError {
    ItemNotFound
}

//Item of merchandise, for transfer, uses stocked and stored
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Item {
    pub name: String,
    pub price: f64,
    stocked: u32,
    stored: u32
}

#[derive(Debug, FromForm)]
pub struct AuthItem {
    pub name: String,
    pub price: f64,
    pub stock: i32,
    pub uuid: String
}

impl Item {
    pub fn new(name: String, price: f64, stocked: u32, stored: u32) -> Item {
        Item { name, price, stocked, stored }
    }

    /// Gets the vendor's stock for this item
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current item object
    pub fn get_count(&self) -> u32 { self.stocked }

    /// Gets the vendor's store for this item
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current item object
    pub fn get_stored(&self) -> u32 { self.stored }

    fn sell_item(&mut self, count: u32) -> u32 {
        if self.stocked >= count {
            self.stocked -= count;
            0
        } else {
            let retval = count - self.stocked;
            self.stocked = 0;
            retval
        }
    }

    fn stock_item(&mut self, count: u32) {
        if count <= self.stored {
            self.stocked += self.stored - count;
            self.stored -= count;
        } else {
            self.stocked += self.stored;
            self.stored = 0;
        }
    }

    fn store_item(&mut self, count: u32) {
        self.stored += count;
    }

    fn total(&self) -> u32 { self.stocked + self.stored }

    fn update(&mut self, price: f64, count: i32) {
        self.price = price;
        if count > 0 {
            let diff = std::cmp::min(self.stored, count as u32);
            self.stored -= diff;
            self.stocked += diff;
        } else {
            let diff = std::cmp::min(self.stocked, (-count) as u32);
            self.stocked -= diff;
            self.stored += diff;
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.price == other.price &&
        self.stocked == other.stocked &&
        self.stored == other.stored
    }
}

//A single vendor in the market
#[derive(Serialize, Deserialize, Clone)]
pub struct Vendor {
    pub name: String,
    pub url: String,
    pub bits: f64,
    pub items: Vec<Item>
}

impl Vendor {
    pub fn new(name: String, url: String, bits: f64) -> Vendor{
        Vendor{ name, url, bits, items: vec![] }
    }

    /// Adds an item to the vendor
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current vendor object
    pub fn add_item(&mut self, item: Item, store: bool) {
        if let Some(i) = self.grab_item(&item.name) {
            i.store_item(item.total());
            if !store { i.stock_item(item.total()) };
        } else {
            self.items.push(item);
        }
    }

    /// Returns true if this vendor has the object
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current ledger object
    /// * `name`    - The name of the desired item
    pub fn contains(&mut self, name: &String) -> bool {
        match self.get_item(name) {
            Some(_) => true,
            None => false
        }
    }

    /// Returns an immutable reference to a vendor's item if it exists
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current ledger object
    pub fn get_item(&self, name: &String) -> Option<&Item> {
        self.items.iter().find(|i| &i.name == name)
    }

    /// Returns a clone of all of the itmes the vendor hass
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current ledger object
    pub fn get_items(&self) -> Vec<Item> {
        self.items.clone()
    }

    /// Purchases a stocked item from a vendor, and return the understock, or
    /// the amount the vendor was not able to fulfill with their stocked goods
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current ledger object
    pub fn purchase_item(&mut self, item: &String, count: u32) -> Result<u32, ShopError> {
        if let Some(i) = self.grab_item(item) {
            Ok(i.sell_item(count))
        } else {
            Err(ShopError::ItemNotFound)
        }
    }

    /// Updates the price and the counts of the item
    /// 
    /// # Arguments
    /// 
    /// * `self`    - The current ledger object
    /// * `item`    - The name of the item to update
    /// * `price`   - The new price of the item
    /// * `count`   - The change from store to stock
    pub fn update_item(&mut self, item: String, price: f64, count: i32) {
        if let Some(i) = self.grab_item(&item) { i.update(price, count); }
    }

    fn grab_item(&mut self, name: &String) -> Option<&mut Item> {
        self.items.iter_mut().find(|i| &i.name == name)
    }
}

impl PartialEq for Vendor {
    fn eq(&self, other: &Self) -> bool{
        self.name == other.name &&
        self.url == other.url &&
        self.bits == other.bits &&
        self.items.len() == other.items.len() &&
        self.items.iter()
                  .zip(&other.items)
                  .all(|(a, b)| a == b)
    }
}

impl fmt::Debug for Vendor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vendor")
         .field("Name", &self.name)
         .field("Url", &self.url)
         .field("Bits", &self.bits)
         .field("Item Count", &self.items.len())
         .field("Items", &self.items)
         .finish()
    }
}

/// A function for updating the state of an object through changing the price
/// and moving units from the stock to the store or vice versa
/// 
/// # Arguments
/// 
/// `auth_item` - The auth item change request
/// `ledger`    - The current ledger state
fn stock(auth_item: AuthItem, ledger: State<super::ledger::MutLedger>) -> BTreeMap<String, Box<dyn Display>> {
    let arc_ledger = ledger.inner().session_ledger.clone();
    let vendor_id: usize;

    let mut output_vars: BTreeMap<String, Box<dyn Display>> = BTreeMap::new();

    {
        let ledger = &*arc_ledger.read().unwrap();
        vendor_id = match ledger.verify_uuid(auth_item.uuid) {
            Ok(id) => id,
            Err(_) => {
                output_vars.insert("success".to_string(), Box::new(false));
                output_vars.insert("UUID".to_string(), Box::new("not recognized".to_string()));
                return output_vars;
            }
        };
    }

    {
        let mut ledger = (&*arc_ledger).write().unwrap();
        ledger.update_item(vendor_id, auth_item.name, auth_item.price, auth_item.stock);
    }

    output_vars.insert("success".to_string(), Box::new(true));
    output_vars
}

/// An endpoint that displays all of the vendors currently in the market with
/// the prices of their goods
/// 
/// # Arguments
/// 
/// * `ledger`    - The current ledger state
#[get("/")]
pub fn market_home(ledger: State<MutLedger>) -> Template {
    let mut map = super::HashMap::new();
    let arc_ledger = ledger.inner().session_ledger.clone();
    let ledger = &*arc_ledger.read().unwrap();
    map.insert("vendor_names", to_value(ledger.get_vendor_names()).unwrap());
    map.insert("vendor_urls", to_value(ledger.get_vendor_urls()).unwrap());
    map.insert("ledger_state", to_value(ledger.serialize_state()).unwrap());
    map.insert("ticker_items", to_value(vec!["All purchases are final!",
                                             "Stocked items are available for synchronous sale!",
                                             "Please keep your hands and feet inside tht market at all times",
                                             "Wear a mask."]).unwrap());
    Template::render("market", &map)
}

/// An endpoint for individual vendors
/// 
/// # Arguments
/// 
/// * `url`       - The url of the vendor
/// * `ledger`    - The current ledger state
#[get("/<url>")]
pub fn vendor(url: String, ledger: State<MutLedger>) -> Template {
    let arc_ledger = ledger.inner().session_ledger.clone();
    let ledger = &*arc_ledger.read().unwrap();
    let vendors = ledger.get_vendors();
    let vend = vendors.iter().find(|x| x.url == url);
    match vend {
        Some(v) => {
            let mut map = super::HashMap::new();
            map.insert("vendor", &v);
            Template::render("vendor", map)
        }
        None => {
            let mut map = super::HashMap::new();
            map.insert("path", &url);
            Template::render("error/404", map)
        }
    }
}

/// Endpoint for making stock orders via HTTP request
/// 
/// # Arguments
/// 
/// * `auth_item`   - The DTO for the stock order being completed
/// * `ledger`      - The current ledger state
#[post("/stock", data="<auth_item>")]
pub fn http_stock(auth_item: Result<Form<AuthItem>, FormError<'_>>, ledger: State<super::ledger::MutLedger>) -> content::Json<String> {
    match auth_item {
        Ok(ai) => super::util::construct_json(&stock(ai.into_inner(), ledger)),
        Err(_) => {
            let mut output_vars: BTreeMap<String, Box<dyn Display>> = BTreeMap::new();
            output_vars.insert("Format".to_string(), Box::new("incorrect"));
            super::util::construct_json(&output_vars)
        }
    }
}

/// Endpoint for manual stock orders using a form
/// 
/// # Arguments
/// 
/// * `auth_item`   - The DTO for the stock order being completed
/// * `ledger`      - The current ledger state
#[post("/stock", data="<auth_item>")]
pub fn form_stock(auth_item: Result<Form<AuthItem>, FormError<'_>>, ledger: State<super::ledger::MutLedger>) -> Template {
    let mut map = super::HashMap::new();
    let mut response = match auth_item {
        Ok(ai) => stock(ai.into_inner(), ledger),
        Err(_) => {
            map.insert("errors", vec!["AuthItem was not filled out".to_string()]);
            return Template::render("stock", &map)
        }
    };
    for (k, v) in response.iter_mut() {
        map.insert(k, vec![format!("{}", *v)]);
    }
    Template::render("stock_response", &map)
}

/// Stocking page GET endpoint
#[get("/stock")]
pub fn stock_page(ledger: State<super::ledger::MutLedger>) -> Template {
    let mut map = super::HashMap::new();
    let arc_ledger = ledger.inner().session_ledger.clone();
    let ledger = &*arc_ledger.read().unwrap();
    map.insert("names", to_value(ledger.get_vendor_names()).unwrap());
    map.insert("urls", to_value(ledger.get_vendor_urls()).unwrap());
    map.insert("ledger_state", to_value(ledger.serialize_state()).unwrap());
    Template::render("stock", &map)
}