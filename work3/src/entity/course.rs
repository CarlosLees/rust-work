use crate::entity::student::Student;

#[derive(Debug)]
pub struct Course {

    pub id: String,

    pub name: String,

    pub students: Vec<Student>
}