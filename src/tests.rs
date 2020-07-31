use rocket::http::{ContentType, Status};
use rocket::local::Client;
use super::*;
use super::shop::{Item, Vendor};

fn create_test_ledger(generate: usize) -> ledger::MutLedger {
    let mut session_ledger = ledger::Ledger::new();
    for _ in 0..generate {
        let _ = session_ledger.register_vendor(util::name_generator(), None);
    }
    ledger::MutLedger{session_ledger: Arc::new(RwLock::new(session_ledger))}
}

#[test]
fn test_index_endpoint() {
    let mut session_ledger = ledger::Ledger::new();
    session_ledger.register_vendor("test".to_string(), None).expect("vendor registered successfully");
    let rocket = rocket::ignite()
                        .manage(create_test_ledger(1))
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
fn test_register_endpoint() {
    let rocket = rocket::ignite()
                        .manage( create_test_ledger(0) )
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