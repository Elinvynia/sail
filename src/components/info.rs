#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Info {
    pub name: String,
    pub description: String,
}

impl Info {
    pub fn new(name: &str, description: &str) -> Self {
        let name = name.to_owned();
        let description = description.to_owned();
        Info { name, description }
    }
}
