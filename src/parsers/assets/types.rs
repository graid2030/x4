use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Ctx {
    pub tag: String,
    pub attrs: HashMap<String, String>,
}
