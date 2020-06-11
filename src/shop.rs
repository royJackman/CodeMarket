#[derive(Debug)]
pub struct Item {
    pub name: &'static str,
    pub price: f64,
    count: u32
}

impl Item {
    pub fn get_count(&self) -> u32 { self.count }
    
    pub fn new(name: &'static str, price: f64, count: u32) -> Item {
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

pub struct Vendor {
    pub name: &'static str,
    pub bits: u32,
    pub items: Vec<Item>
}

impl Vendor {
    pub fn new(name: &'static str, bits: u32) -> Vendor{
        Vendor{ name, bits, items: vec![] }
    }

    fn contains(&mut self, name: &'static str) -> Option<&mut Item> {
        self.items.iter_mut().find(|i| i.name == name)
    }

    pub fn add_item(&mut self, item: Item) {
        if let Some(i) = self.contains(item.name) {
            i.stock_item(item.get_count());
        } else {
            self.items.push(item);
        }
    }

    pub fn get_item(&self, name: &'static str) -> Option<&Item> {
        self.items.iter().find(|i| i.name == name)
    }

    pub fn purchase_item(&mut self, item: &'static str, count: u32) {
        if let Some(i) = self.contains(item) {
            i.sell_item(count)
        }
    }
}

impl PartialEq for Vendor {
    fn eq(&self, other: &Self) -> bool{
        self.name == other.name &&
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
         .field("Bits", &self.bits)
         .field("Item Count", &self.items.len())
         .field("Items", &self.items)
         .finish()
    }
}