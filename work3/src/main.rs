//基于Rust的基本数据结构写一个简单的学生管理系统（比如，学生，社团，班级、课程等），明确类型之间的关系，进行基本的CRUD操作。

use std::collections::HashMap;
use clap::Parser;
use dialoguer::{MultiSelect, Select};
use dialoguer::theme::ColorfulTheme;
use crate::cli::{Action, Arg};
use crate::config::application_config::ApplicationConfig;
use crate::entity::class::Class;
use crate::entity::club::Club;
use crate::entity::student::Student;

mod entity;
mod config;
mod cli;

fn main() {
    let arg = Arg::parse();

    match arg.action {
        Action::Start => start_manage(),
    }
}

fn start_manage() {
    let mut studentMap:HashMap<String, Student> = HashMap::new();
    let mut clubMap:HashMap<String,Club> = HashMap::new();
    let mut classMap:HashMap<String,Class> = HashMap::new();

    let config = get_config_from_yaml();

    let theme = ColorfulTheme::default();

    loop {
        println!("学生数量:{}",studentMap.len());
        if !studentMap.is_empty() {
            for (id,student) in studentMap.iter() {
                print!("学号:{:?},姓名:{:?},",id,&student.name);
                if let Some(club_id) = &student.club_id {
                    print!("社团:{:?}",clubMap.get(club_id.as_str()))
                }

                if let Some(class_id) = &student.class_id {
                    print!("班级:{:?}",classMap.get(class_id.as_str()))
                }
                println!();
            }
        }

        let x = Select::with_theme(&theme).with_prompt("欢迎进入学生管理系统，请选择服务:").items(&config
            .clap_select).interact().unwrap();

    }
}

fn get_config_from_yaml() -> ApplicationConfig {
    let content = include_str!("../application.yml");
    let config:ApplicationConfig = serde_yaml::from_str(&content).unwrap();
    config
}