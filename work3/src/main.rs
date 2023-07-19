//基于Rust的基本数据结构写一个简单的学生管理系统（比如，学生，社团，班级、课程等），明确类型之间的关系，进行基本的CRUD操作。

use std::collections::HashMap;
use clap::Parser;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use crate::cli::{Action, Arg};
use crate::config::application_config::ApplicationConfig;
use crate::entity::class::Class;
use crate::entity::club::Club;
use crate::entity::course::Course;
use crate::entity::student::Student;

mod entity;
mod config;
mod cli;
mod service;

#[derive(Debug)]
pub struct Store {
    pub student_store: HashMap<String,Student>,
    pub class_store: HashMap<String,Class>,
    pub source_store: HashMap<String,Course>,
    pub club_store: HashMap<String,Club>,
}

impl Store {
    fn new() -> Self {
        Self {
            student_store: HashMap::new(),
            class_store: HashMap::new(),
            source_store: HashMap::new(),
            club_store: HashMap::new(),
        }
    }
}

fn main() {
    let arg = Arg::parse();

    match arg.action {
        Action::Start => start_manage(),
    }
}

fn get_config_from_yaml() -> ApplicationConfig {
    let content = include_str!("../application.yml");
    let config:ApplicationConfig = serde_yaml::from_str(&content).unwrap();
    config
}


fn start_manage() {
    //获取服务分类
    let config = get_config_from_yaml().clap_select;
    //初始化主题
    let theme = ColorfulTheme::default();
    let items = vec![&config.student.name,&config.class.name,&config.course.name,&config.club.name];
    //初始化存储
    let mut store = Store::new();

    loop {
        let select = Select::with_theme(&theme).with_prompt("欢迎进入学生管理系统，请选择服务:").items(&items).interact().unwrap();

        match select {
            0 => {
                //学生服务(已完成)
                service::student_service::student_service(&mut store,&theme,&config.student);
            },
            1 => {
                service::class_service::class_service(&mut store,&theme,&config.class);
            },
            2 => {},
            3 => {},
            _ => {}
        }
    }
}

