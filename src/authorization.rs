#[derive(Deserialize, Debug)]
pub struct RegistrationData<'a> {
    pub vendor_name: &'a str,
}