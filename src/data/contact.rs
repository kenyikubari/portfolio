use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Contact {
    pub name: String,
    pub biography: String,
    pub youtube: Option<String>,
    pub instagram: Option<String>,
    pub vsco: Option<String>,
    pub research_gate: Option<String>,
    pub tiktok: Option<String>,
    pub github: Option<String>,
    pub linkedin: Option<String>,
}

#[derive(Deserialize)]
struct ContactFile {
    contact: Vec<Contact>,
}

pub fn get_contact() -> Contact {
    let toml_str = include_str!("../../assets/data/contact.toml");
    let file: ContactFile = toml::from_str(toml_str).expect("Failed to parse contact.toml");
    file.contact.into_iter().next().expect("No contact entry found")
}