use rocket::State;
use rocket_contrib::templates::Template;
use super::ledger::MutLedger;

pub enum ShopError {
    ItemNotFound
}

//Item of merchandise, for transfer, uses stocked and stored
#[derive(Debug, Serialize, Deserialize, Clone)]
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

    pub fn contains(&mut self, name: &String) -> bool {
        match self.get_item(name) {
            Some(_) => true,
            None => false
        }
    }

    pub fn add_item(&mut self, item: Item, store: bool) {
        if let Some(i) = self.grab_item(&item.name) {
            i.store_item(item.total());
            if !store { i.stock_item(item.total()) };
        } else {
            self.items.push(item);
        }
    }

    pub fn get_item(&self, name: &String) -> Option<&Item> {
        self.items.iter().find(|i| &i.name == name)
    }

    pub fn get_items(&self) -> Vec<Item> {
        self.items.clone()
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

//Index endpoint for all vendors
#[get("/")]
pub fn market_home(ledger: State<MutLedger>) -> Template {
    let mut map = super::HashMap::new();
    let arc_ledger = ledger.inner().session_ledger.clone();
    let ledger = &*arc_ledger.read().unwrap();
    let vendor_names = ledger.get_vendor_names();
    map.insert("vendor_names", vendor_names.clone());
    map.insert("vendor_urls", ledger.get_vendor_urls());
    
    let mut all_items = vec![];
    let mut all_prices = vec![];
    let mut item_count = vec![];
    for i in 0..vendor_names.len() {
        let temp_items = ledger.get_vendor_items(i);
        item_count.push(temp_items.len().to_string());
        for item in ledger.get_vendor_items(i) {
            all_items.push(item.name.clone());
            all_prices.push(item.price.clone().to_string());
        }
    }
    map.insert("all_items", all_items);
    map.insert("all_prices", all_prices);
    map.insert("item_count", item_count);
    Template::render("market", &map)
}

//Individual vendor page
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