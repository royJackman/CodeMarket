use std::io::{self, Read};
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
        Success(OrderData{
            item: &splits[0].split(" ").collect::<Vec<&str>>().last().unwrap(), 
            count: &splits[1].split(" ").collect::<Vec<&str>>().last().unwrap(), 
            from: &splits[2].split(" ").collect::<Vec<&str>>().last().unwrap(), 
            to: &splits[3].split(" ").collect::<Vec<&str>>().last().unwrap()
        })
    }
}

#[post("/purchase", format="application/json", data="<order_data>")]
pub fn purchase(order_data: OrderData, market: State<super::Market>) -> content::Json<String> {
    let order = Order::from_data(order_data);
    for v in &market.vendors {
        match v.get_item(&order.item) {
            Some(i) => println!("Found {} at {} for {}", i.name, v.name, i.price),
            None => println!("Did not find {} at {}", order.item, v.name)
        }
    }

    content::Json(format!(
        "{} \"item\": \"{}\", \"count\": \"{}\", \"from\": \"{}\", \"to\":\"{}\" {}",
        "{", order.item, order.count, order.from, order.to, "}"
    ))
}