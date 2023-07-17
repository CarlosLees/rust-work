//基于Rust的基本数据结构写一个简单的学生管理系统（比如，学生，社团，班级、课程等），明确类型之间的关系，进行基本的CRUD操作。

use entity::student;
fn main() {
    let entity = student::ActiveModel {
        id: Default::default(),
        name: Default::default(),
        age: Default::default(),
        class_id: Default::default(),
        club_id: Default::default(),
    };
    println!("Hello, world! {:?}",entity);
}
