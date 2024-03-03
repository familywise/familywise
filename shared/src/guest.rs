#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Guest {
    pub id: uuid::Uuid,
    pub name: String,
    pub hash: String,
}

impl Guest {
    pub fn new(name: &str, pass: &str) -> Self {
        let id = uuid::Uuid::new_v4();
        Guest {
            id,
            name: name.to_owned(),
            hash: pass.to_owned(),
        }
    }
}
