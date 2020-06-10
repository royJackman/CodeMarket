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

    fn stock_item(&mut self, count: u32){
        self.count += count
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

    fn contains(&mut self, item:&Item) -> Option<&mut Item> {
        self.items.iter_mut().find(|i| i.name == item.name)
    }

    pub fn add_item(&mut self, item: Item) {
        if let Some(i) = self.contains(&item) {
            i.stock_item(item.get_count());
        } else {
            self.items.push(item);
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