// use syn;

#[derive(Debug)]
pub struct CWItem {
    pub item_type: String,
    pub name: String,
}


pub fn from_item(name: String) -> CWItem {
    CWItem {
        item_type: "test".to_string(),
        name: name,
    }
}
