#[derive(Debug, Clone)]
pub struct Dex {
    pub name: String,
    pub router_address: String,
}

impl Dex {
    pub fn new(name: &str, router_address: &str) -> Self {
        Dex {
            name: name.to_string(),
            router_address: router_address.to_string(),
        }
    }
}