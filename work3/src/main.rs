//基于Rust的基本数据结构写一个简单的学生管理系统（比如，学生，社团，班级、课程等），明确类型之间的关系，进行基本的CRUD操作。

use clap::Parser;
use dialoguer::Input;
use dialoguer::theme::ColorfulTheme;
use crate::cli::{Action, Arg};
use crate::entity::student::Student;

mod entity;
mod cli;

fn main() {
    let arg = Arg::parse();

    match arg.action {
        Action::Start => start_manage(),
    }
}

fn start_manage() {
    let mut students:Vec<Student> = Vec::new();

    let theme = ColorfulTheme::default();

    loop {
        let x:String = Input::with_theme(&theme).with_prompt("请选择").interact_text().unwrap();
        println!("{}", x);
    }
}