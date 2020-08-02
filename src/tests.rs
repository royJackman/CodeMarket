use rocket::http::{ContentType, Status};
use rocket::local::Client;
use std::collections::HashSet;
use std::iter::FromIterator;
use super::*;
use super::shop::{Item, Vendor};

fn create_test_ledger(generate: usize) -> (ledger::MutLedger, Vec<String>) {
    let mut session_ledger = ledger::Ledger::new();
    let mut ids = vec![];
    for _ in 0..generate {
        ids.push(session_ledger.register_vendor(util::name_generator(), None).unwrap());
    }
    (ledger::MutLedger{session_ledger: Arc::new(RwLock::new(session_ledger))}, ids)
}

#[test]
fn test_index_endpoint() {
    let mut session_ledger = ledger::Ledger::new();
    session_ledger.register_vendor("test".to_string(), None).expect("vendor registered successfully");
    let rocket = rocket::ignite()
                        .manage(create_test_ledger(1).0)
                        .mount("/", StaticFiles::from("templates"))
                        .mount("/", routes![base::index])
                        .attach(Template::custom(|engines| {
                            let var = BTreeMap::new();
                            engines.tera.register_function("get_rust_type_index", tera_functions::make_get_rust_type_index(var.clone()));
                        }));
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::HTML));
    assert!(response.body_string().unwrap().contains("The Code Market"));
}

#[test]
fn test_get_item_history() {
    let mut ledger = ledger::Ledger::new();
    let _ = ledger.register_vendor("test".to_string(), None);
    for s in util::get_rust_types(0) {
        if !ledger.get_ledger_items().contains(&s.to_string()) {
            assert_eq!(ledger.get_item_history(s.to_string()), vec![0.0]);
        }
    }
}

#[test]
fn test_get_ledger_items() {
    let mut ledger = ledger::Ledger::new();
    let _ = ledger.register_vendor("test".to_string(), None);
    let vendor_items: Vec<String> = ledger.get_vendor(0)
                                          .get_items()
                                          .iter()
                                          .map(|x| x.name.clone())
                                          .collect();
    let ledger_items: Vec<String> = ledger.get_ledger_items();
    let vi: HashSet<&String> = HashSet::from_iter(vendor_items.iter());
    let li: HashSet<&String> = HashSet::from_iter(ledger_items.iter());
    assert_eq!(vi, li);
}

#[test]
fn test_get_price_history() {
    let mut ledger = ledger::Ledger::new();
    let _ = ledger.register_vendor("test".to_string(), None);
    let ledger_items: Vec<String> = ledger.get_ledger_items();
    let price_history = ledger.get_price_history();
    for rt in util::get_rust_types(0) {
        if !ledger_items.contains(&rt.to_string()) {
            assert_eq!(price_history[util::get_rust_type_index(rt.to_string())], vec![0.0]);
        }
    }
}

#[test]
fn test_get_vendor() {
    let mut ledger = ledger::Ledger::new();
    let _ = ledger.register_vendor("test".to_string(), None);
    let v = ledger.get_vendor(0);
    assert_eq!("test".to_string(), v.name);
}

#[test]
fn test_get_vendors() {
    let mut ledger = ledger::Ledger::new();
    let _ = ledger.register_vendor("test".to_string(), None);
    let _ = ledger.register_vendor("test2".to_string(), None);
    let vs = ledger.get_vendors();
    assert_eq!(vs.len(), 2);
}

#[test]
fn test_get_vendor_names() {
    let mut ledger = ledger::Ledger::new();
    let _ = ledger.register_vendor("test".to_string(), None);
    let _ = ledger.register_vendor("test2".to_string(), None);
    let vn = ledger.get_vendor_names();
    assert!(vn.contains(&"test".to_string()));
    assert!(vn.contains(&"test2".to_string()));
}

#[test]
fn test_get_vendor_urls() {
    let mut ledger = ledger::Ledger::new();
    let _ = ledger.register_vendor("test".to_string(), None);
    let _ = ledger.register_vendor("test2".to_string(), Some("test".to_string()));
    let vn = ledger.get_vendor_names();
    assert!(vn.contains(&"test".to_string()));
    assert_eq!(vn.len(), 1);
}

#[test]
fn test_get_version() {
    let mut ledger = ledger::Ledger::new();
    let _ = ledger.register_vendor("test".to_string(), None);
    assert_eq!(ledger.get_version() as usize, ledger.get_ledger_items().len());
}

#[test]
fn test_new_item() {
    let i = Item::new(String::from("test_item"), 1.0, 32, 32);
    assert_eq!(&i.name[..], "test_item");
    assert_eq!(i.price, 1.0);
    assert_eq!(i.get_count(), 32);
}

#[test]
fn test_new_vendor() {
    let v = Vendor::new(String::from("Test Vendor"), String::from("test_vendor"), 100.0);
    assert_eq!(&v.name[..], "Test Vendor");
    assert_eq!(v.bits, 100.0);
    assert_eq!(v.items, vec![]);
}

#[test]
fn test_purchase() {
    let mut ledger = ledger::Ledger::new();
    let id1 = ledger.register_vendor("test".to_string(), None).unwrap();
    let v1_items = ledger.get_ledger_items();
    let id2 = ledger.register_vendor("test2".to_string(), None).unwrap();

    let rocket = rocket::ignite()
                        .manage(ledger::MutLedger{ session_ledger: Arc::new(RwLock::new(ledger)) })
                        .mount("/", routes![purchase::http_purchase, shop::http_stock]);
    let client = Client::new(rocket).expect("valid rocket instance");
    
    let mut stock_response = client.post("/stock")
                                   .body(format!(
                                       "name={}&price={}&stock={}&uuid={}", 
                                       v1_items[0].clone(), 1.0, 5, id1.clone()
                                    ))
                                   .header(ContentType::Form)
                                   .dispatch();
    assert_eq!(stock_response.status(), Status::Ok);
    assert_eq!(stock_response.content_type(), Some(ContentType::JSON));
    assert!(stock_response.body_string().unwrap().contains("\"success\": \"true\""));

    let mut purchase_response = client.post("/purchase")
                                      .body(format!(
                                          "item={}&count={}&from={}&to={}",
                                          v1_items[0].clone(), 5, "test", id2
                                        ))
                                      .header(ContentType::Form)
                                      .dispatch();
    assert_eq!(purchase_response.status(), Status::Ok);
    assert_eq!(purchase_response.content_type(), Some(ContentType::JSON));
    assert!(purchase_response.body_string().unwrap().contains("\"success\": \"true\""));
}

#[test]
fn test_register_endpoint() {
    let rocket = rocket::ignite()
                        .manage( create_test_ledger(0).0 )
                        .mount("/", routes![authorization::register]);
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut response = client.post("/register")
                             .body("vendor_name=vendor&vendor_url=vendor")
                             .header(ContentType::Form)
                             .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert!(response.body_string().unwrap().contains("\"uuid\""));
}

#[test]
fn test_register_vendor() {
    let mut ledger = ledger::Ledger::new();
    let _ = ledger.register_vendor("Test Name".to_string(), None);
    assert!(ledger.get_ledger_items().len() > 0);
    assert_eq!(ledger.get_vendor_names(), ["Test Name".to_string()]);
    assert_eq!(ledger.get_vendor_urls(), ["test_name".to_string()]);
}

#[test]
fn test_register_endpoint_partial() {
    let rocket = rocket::ignite()
                        .manage( ledger::MutLedger{session_ledger: Arc::new(RwLock::new(ledger::Ledger::new()))} )
                        .mount("/", routes![authorization::register]);
    let client = Client::new(rocket).expect("valid rocket instance");
    let mut response = client.post("/register")
                             .body("vendor_name=vendor&vendor_url=")
                             .header(ContentType::Form)
                             .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert!(response.body_string().unwrap().contains("\"uuid\""));
}

#[test]
fn test_serialize_state() {
    let mut ledger = ledger::Ledger::new();
    let empty_ledger = HashMap::new();
    assert_eq!(ledger.serialize_state(), empty_ledger);
    let _ = ledger.register_vendor("test".to_string(), None);
    assert!(ledger.serialize_state().contains_key("test"));
}

#[test]
fn test_serialize_vendor() {
    let mut ledger = ledger::Ledger::new();
    let _ = ledger.register_vendor("test".to_string(), None);
    let sv = ledger.serialize_vendor(0);
    assert_eq!(sv.1.len(), 0);
    assert_eq!(sv.0.len(), sv.2.len());
}

#[test]
fn test_vendor_add_item() {
    let v = &mut Vendor::new(String::from("Vendor"), String::from("vendor"), 1000.0);
    v.add_item(Item::new(String::from("f32"), 32.0, 100, 100), false);
    let item = v.get_item(&String::from("f32")).unwrap();
    assert_eq!(item.price, 32.0);
    assert_eq!(item.get_count(), 100);
}

#[test]
fn test_vendor_purchase_item() {
    let v = &mut Vendor::new(String::from("Vendor"), String::from("vendor"), 1000.0);
    let f32 = String::from("f32");
    let u8 = String::from("u8");
    let stir = String::from("str");
    v.add_item(Item::new(f32.clone(), 32.0, 100, 100), false);
    v.add_item(Item::new(u8.clone(), 8.0, 100, 100), false);
    v.add_item(Item::new(stir.clone(), 1.0, 40, 40), false);
    let _ = v.purchase_item(&u8, 70);
    let _ = v.purchase_item(&stir, 50);
    assert_eq!(30, v.get_item(&u8).unwrap().get_count());
    assert_eq!(0, v.get_item(&stir).unwrap().get_count());
}

#[test]
fn test_verify_uuid() {
    let (ledger, ids) = create_test_ledger(3);
    let arc_ledger = ledger.session_ledger.clone();
    {
        let reader = &*arc_ledger.read().unwrap();
        for i in ids {
            assert!(match reader.verify_uuid(i) {
                Ok(_) => true,
                Err(_) => false
            } );
        }
    }
}