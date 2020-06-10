use super::shop::{Item, Vendor};

#[test]
fn test_new_item() {
    let i = Item::new("test_item", 1.0, 32);
    assert_eq!(i.name, "test_item");
    assert_eq!(i.price, 1.0);
    assert_eq!(i.get_count(), 32);
}

#[test]
fn test_new_vendor() {
    let v = Vendor::new("test_vendor", 100);
    assert_eq!(v.name, "test_vendor");
    assert_eq!(v.bits, 100);
    assert_eq!(v.items, vec![]);
}

#[test]
fn test_vendor_add_item() {
    let v = &mut Vendor::new("stocked_vendor", 1000);
    v.add_item(Item::new("f32", 32.0, 100));
    let item = v.items.iter()
                      .find(|i| i.name == "f32")
                      .unwrap();
    assert_eq!(item.price, 32.0);
    assert_eq!(item.get_count(), 100);
}