use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct ApplicationConfig {
    pub clap_select: ClapSelect
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ClapSelect {
    pub student: StudentConfig,
    pub class: ClassConfig,
    pub club: ClubConfig,
    pub course: CourseConfig
}

#[derive(Debug,Serialize,Deserialize)]
pub struct StudentConfig {
    pub name: String,
    pub options: Vec<String>
}

#[derive(Debug,Deserialize,Serialize)]
pub struct ClassConfig {
    pub name: String,
    pub options: Vec<String>
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ClubConfig {
    pub name: String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct CourseConfig {
    pub name: String
}

