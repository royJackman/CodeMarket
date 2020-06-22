use std::io::{self, Read};
use std::fmt::Display;
use std::collections::BTreeMap;

use rocket::response::content;
use rocket::{State, Request, Data, Outcome::*};
use rocket::data::{FromData, Outcome, Transform, Transformed};
use rocket::http::Status;

const BUFFER_SIZE: u64 = 256;

pub enum OrderError {
    Io(io::Error),
    Parse
}

pub struct Order {
    pub item: String,
    pub count: u32,
    pub from: String,
    pub to: String
}

impl Order {
    pub fn from_data(data: OrderData) -> Order {
        let clean_string = |string: &str| String::from(string).replace("\"", "").replace(",", "").replace("\r", "");
        Order {
            item: clean_string(data.item),
            count: clean_string(data.count).parse().unwrap(),
            from: clean_string(data.from),
            to: clean_string(data.to)
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct OrderData<'a> {
    pub item: &'a str,
    pub count: &'a str,
    pub from: &'a str,
    pub to: &'a str
}

impl<'a> FromData<'a> for OrderData<'a> {
    type Error = OrderError;
    type Owned = String;
    type Borrowed = str;

    fn transform(_: &Request, data: Data) -> Transform<Outcome<Self::Owned, Self::Error>> {
        let mut stream = data.open().take(BUFFER_SIZE);
        let mut string = String::with_capacity((BUFFER_SIZE/4) as usize);
        let outcome = match stream.read_to_string(&mut string) {
            Ok(_) => Success(string),
            Err(e) => Failure((Status::InternalServerError, OrderError::Io(e)))
        };

        Transform::Borrowed(outcome)
    }

    fn from_data(_: &Request, outcome: Transformed<'a, Self>) -> Outcome<Self, Self::Error> {
        let string = outcome.borrowed()?;
        let tabboo: Vec<&str> = vec![&r"{", &r"}"];
        let whitespace: Vec<&str> = vec![&"", &"\n"];
        let splits: Vec<&str> = string.trim()
                                      .split("\n")
                                      .filter(move |x| {
                                          for t in tabboo.iter() {
                                              if x.contains(t) { return false }
                                          }
                                          for w in whitespace.iter() {
                                              if x == w { return false }
                                          }
                                          true
                                      })
                                      .collect();
        let to_remove: &[_] = &[',', '\n'];
        Success(OrderData{
            item: &splits[0].trim().trim_matches(to_remove).split("\"").filter(|x| x != &"").collect::<Vec<&str>>().last().unwrap(), 
            count: &splits[1].trim().trim_matches(to_remove).split("\"").filter(|x| x != &"").collect::<Vec<&str>>().last().unwrap(), 
            from: &splits[2].trim().trim_matches(to_remove).split("\"").filter(|x| x != &"").collect::<Vec<&str>>().last().unwrap(), 
            to: &splits[3].trim().trim_matches(to_remove).split("\"").filter(|x| x != &"").collect::<Vec<&str>>().last().unwrap()
        })
    }
}

#[post("/purchase", format="application/json", data="<order_data>")]
pub fn purchase(order_data: OrderData, ledger: State<super::ledger::MutLedger>) -> content::Json<String> {
    let order = Order::from_data(order_data);
    let arc_ledger = ledger.inner().session_ledger.clone();
    let ledger = &*arc_ledger.write().unwrap();
    let buyer_pos = ledger.verify_uuid(order.to).unwrap_or(usize::MAX);
    let mut seller_pos = usize::MAX;
    let mut vendors = ledger.vendors.lock().unwrap();
    let mut output_vars: BTreeMap<String, Box<dyn Display>> = BTreeMap::new();

    for (i,v) in vendors.iter().enumerate() {
        if v.name == order.from { seller_pos = i }
    }

    if seller_pos == usize::MAX {
        output_vars.insert("seller".to_string(), Box::new("not found".to_string()));
    }
    if buyer_pos == usize::MAX {
        output_vars.insert("buyer".to_string(), Box::new("not found".to_string()));
    }
    if output_vars.len() > 0 {
        return super::util::construct_json(&output_vars)
    }

    let mut item_price: f64 = 0.0;
    let mut item_count: u32 = 0;
    let item_found: bool;
    let seller_name: String;
    {
        let from = vendors.get(seller_pos).unwrap();
        seller_name = from.name.clone();
        item_found = match from.get_item(&order.item) {
            Some(i) => {
                item_price = i.price;
                item_count = match i.get_count() {
                    0 => {
                        output_vars.insert("item".to_string(), Box::new("out of stock".to_string()));
                        return super::util::construct_json(&output_vars)
                    }
                    x => x
                };
                true
            }, 
            None => {
                output_vars.insert("item".to_string(), Box::new("not found at seller".to_string()));
                return super::util::construct_json(&output_vars)
            }
        };
    }
    
    let buyer_bits: f64;
    let buyer_name: String;
    {
        let to = vendors.get(buyer_pos).unwrap();
        buyer_bits = to.bits;
        buyer_name = to.name.clone();
    }

    let total = item_price * (order.count as f64);
    if total > buyer_bits {
        output_vars.insert("buyer".to_string(), Box::new("cannot afford the purchase".to_string()));
        return super::util::construct_json(&output_vars)
    }

    let mut understock = 0;
    let mut success = false;

    if item_found && (buyer_bits >= total) && item_count > 0 {
        success = true;
        {
            let from_vendor = vendors.get_mut(seller_pos).unwrap();
            understock = match from_vendor.purchase_item(&order.item, order.count) {
                Ok(u) => u, Err(_) => 0
            }
        }
        {
            let to_vendor = vendors.get_mut(buyer_pos).unwrap();
            to_vendor.add_item(super::shop::Item::new(order.item.clone(), item_price, order.count, 0), false);
            to_vendor.bits -= total - (item_price * (understock as f64));
        }
    }

    output_vars.insert("success".to_string(), Box::new(success));
    output_vars.insert("total".to_string(), Box::new(total));
    output_vars.insert("understock".to_string(), Box::new(understock));
    output_vars.insert("seller".to_string(), Box::new(seller_name));
    output_vars.insert("buyer".to_string(), Box::new(buyer_name));

    super::util::construct_json(&output_vars)
}