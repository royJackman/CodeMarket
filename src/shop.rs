use rocket::State;
use rocket_contrib::templates::Template;

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub price: f64,
    count: u32
}

impl Item {
    pub fn get_count(&self) -> u32 { self.count }
    
    pub fn new(name: String, price: f64, count: u32) -> Item {
        Item { name, price, count }
    }

    fn stock_item(&mut self, count: u32) {
        self.count += count
    }

    fn sell_item(&mut self, count: u32) {
        if self.count >= count {
            self.count -= count
        } else {
            self.count = 0
        }
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name &&
        self.price == other.price &&
        self.count == other.count
    }
}

#[derive(Serialize, Deserialize)]
pub struct Vendor {
    pub name: String,
    pub url: String,
    pub bits: u32,
    pub items: Vec<Item>
}

impl Vendor {
    pub fn new(name: String, url: String, bits: u32) -> Vendor{
        Vendor{ name, url, bits, items: vec![] }
    }

    fn contains(&mut self, name: &String) -> Option<&mut Item> {
        self.items.iter_mut().find(|i| &i.name == name)
    }

    pub fn add_item(&mut self, item: Item) {
        if let Some(i) = self.contains(&item.name) {
            i.stock_item(item.get_count());
        } else {
            self.items.push(item);
        }
    }

    pub fn get_item(&self, name: &String) -> Option<&Item> {
        self.items.iter().find(|i| &i.name == name)
    }

    pub fn purchase_item(&mut self, item: &String, count: u32) {
        if let Some(i) = self.contains(item) {
            i.sell_item(count)
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
pub fn vender_home(market: State<super::Market>) -> Template {
    let mut map = super::HashMap::new();
    map.insert("vendors", &market.vendors);
    Template::render("market", map)
}

#[get("/<url>")]
pub fn vendor(url: String, market: State<super::Market>) -> Template {
    match &market.get_vendor_by_url(&url) {
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