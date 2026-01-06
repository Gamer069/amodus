use serde::Deserialize;

#[derive(Deserialize)]
pub struct ModToml {
    pub meta: Meta,
}

#[derive(Deserialize)]
pub struct Meta {
    pub display_name: String,
    pub display_version: String,
}
