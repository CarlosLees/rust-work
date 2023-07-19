
#[derive(Debug,Clone)]
pub struct Student {

    pub id: String,

    pub name: String,

    pub age: i32,

    pub class_id: Option<String>,

    pub club_id: Option<String>
}

impl Student {
    pub fn new(id:String,name: String,age: i32) -> Self {
        Self {
            id,
            name,
            age,
            class_id:None,
            club_id:None
        }
    }

    pub fn update_name(&mut self,name: String){
        self.name = name;
    }

    pub fn update_age(&mut self,age: i32) {
        self.age = age;
    }
}