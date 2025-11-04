use crate::models::Position;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StackItem {
    pub tag: String,
    pub attrs: HashMap<String, String>,
    pub position: Option<Position>,
}
