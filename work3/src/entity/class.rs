
#[derive(Debug,Clone)]
pub struct Class {

    pub id: String,

    pub name: String

}

impl Class {
    pub fn new(id:String,name: String) -> Self {
        Self {
            id,
            name
        }
    }
}