use super::shop::{Item, Vendor};

#[test]
fn test_new_item() {
    let i = Item::new(String::from("test_item"), 1.0, 32);
    assert_eq!(&i.name[..], "test_item");
    assert_eq!(i.price, 1.0);
    assert_eq!(i.get_count(), 32);
}

#[test]
fn test_new_vendor() {
    let v = Vendor::new(String::from("Test Vendor"), String::from("test_vendor"), 100.0);
    assert_eq!(&v.name[..], "Test Vendor");
    assert_eq!(v.bits, 100);
    assert_eq!(v.items, vec![]);
}

#[test]
fn test_vendor_add_item() {
    let v = &mut Vendor::new(String::from("Vendor"), String::from("vendor"), 1000.0);
    v.add_item(Item::new(String::from("f32"), 32.0, 100));
    let item = v.get_item(&String::from("f32")).unwrap();
    assert_eq!(item.price, 32.0);
    assert_eq!(item.get_count(), 100);
}

#[test]
#[allow(non_snake_case)]
fn test_vendor_purchase_item() {
    let v = &mut Vendor::new(String::from("Vendor"), String::from("vendor"), 1000.0);
    let F32 = String::from("f32");
    let U8 = String::from("u8");
    let STR = String::from("str");
    v.add_item(Item::new(F32.clone(), 32.0, 100));
    v.add_item(Item::new(U8.clone(), 8.0, 100));
    v.add_item(Item::new(STR.clone(), 1.0, 40));
    let _ = v.purchase_item(&U8, 70);
    let _ = v.purchase_item(&STR, 50);
    assert_eq!(30, v.get_item(&U8).unwrap().get_count());
    assert_eq!(0, v.get_item(&STR).unwrap().get_count());
}