use rocket::State;
use rocket_contrib::templates::Template;
use super::ledger::{Ledger, MutLedger};

pub enum ShopError {
    ItemNotFound
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub price: f64,
    stocked: u32,
    stored: u32
}

impl Item {
    pub fn get_count(&self) -> u32 { self.stocked }

    fn total(&self) -> u32 { self.stocked + self.stored }
    
    pub fn new(name: String, price: f64, stocked: u32, stored: u32) -> Item {
        Item { name, price, stocked, stored }
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
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.price == other.price &&
        self.stocked == other.stocked &&
        self.stored == other.stored
    }
}

#[derive(Serialize, Deserialize)]
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

    pub fn contains(&mut self, name: &String) -> bool {
        match self.get_item(name) {
            Some(_) => true,
            None => false
        }
    }

    pub fn add_item(&mut self, item: Item, store: bool) {
        if let Some(i) = self.grab_item(&item.name) {
            if store { i.store_item(item.total()) }
            else { i.stock_item(item.total()) }
        } else {
            self.items.push(item);
        }
    }

    pub fn get_item(&self, name: &String) -> Option<&Item> {
        self.items.iter().find(|i| &i.name == name)
    }

    pub fn grab_item(&mut self, name: &String) -> Option<&mut Item> {
        self.items.iter_mut().find(|i| &i.name == name)
    }

    pub fn purchase_item(&mut self, item: &String, count: u32) -> Result<u32, ShopError> {
        if let Some(i) = self.grab_item(item) {
            Ok(i.sell_item(count))
        } else {
            Err(ShopError::ItemNotFound)
        }
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

impl super::fmt::Debug for Vendor {
    fn fmt(&self, f: &mut super::fmt::Formatter<'_>) -> super::fmt::Result {
        f.debug_struct("Vendor")
         .field("Name", &self.name)
         .field("Url", &self.url)
         .field("Bits", &self.bits)
         .field("Item Count", &self.items.len())
         .field("Items", &self.items)
         .finish()
    }
}

#[get("/")]
pub fn market_home(ledger: State<MutLedger>) -> Template {
    let mut map = super::HashMap::new();
    let arc_ledger = ledger.inner().session_ledger.clone();
    let ledger = &*arc_ledger.read().unwrap();
    map.insert("vendors", &ledger.vendors);
    Template::render("market", map)
}

#[get("/<url>")]
pub fn vendor(url: String, ledger: State<MutLedger>) -> Template {
    let arc_ledger = ledger.inner().session_ledger.clone();
    let ledger = &*arc_ledger.read().unwrap();
    let vendors = ledger.vendors.lock().unwrap();
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