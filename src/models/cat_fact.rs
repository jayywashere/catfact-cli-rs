use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CatFact {
    pub fact: String,
    pub length: u32,
}