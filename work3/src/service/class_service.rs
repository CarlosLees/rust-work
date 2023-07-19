use std::collections::HashMap;
use dialoguer::{Input, Select};
use dialoguer::theme::ColorfulTheme;
use crate::config::application_config::ClassConfig;
use crate::entity::class::Class;
use crate::entity::student::Student;
use crate::Store;

pub fn class_service(store: &mut Store,theme: &ColorfulTheme,class_config: &ClassConfig) {

    let class_options = &class_config.options;

    loop {
        let select = Select::with_theme(theme).with_prompt("请选择班级操作:").items(class_options).interact()
            .unwrap();

        match select {
            0 => {
                //班级列表
                class_list(&store.class_store);
            },
            1 => {
                //添加班级
                add_class(&mut store.class_store,theme);
            },
            2 => {
                //删除班级
                delete_class(&mut store.class_store,theme);
            },
            3 => {
                //添加学生
                add_student(&mut store.class_store,&mut store.student_store,theme);
            },
            4 => {
                //移除学生
            }
            _ => {
                break;
            }
        }
    }

}

fn add_student(class_map: &mut HashMap<String, Class>, student_map: &mut HashMap<String, Student>,
               theme: &ColorfulTheme) {

    //班级信息
    let class_list:Vec<&Class> = class_map.iter().map(|(_,class)| {
        &class
    }).collect();

    //展示学生信息
    if student_map.is_empty() {
        println!("学生信息为空，请先添加学生信息");
        println!("==================================");
        return;
    }

    for (student_id, student) in student_map.iter() {
        println!("学习id为:{:?},学生姓名为:{:?}",student_id,student.name);
    }

    let mut student_id = String::new();

    while student_id.is_empty() {
        student_id = Input::with_theme(theme).with_prompt("请输入要添加的学生id:").interact_text().unwrap();

        if !student_map.contains_key(&student_id) {
            println!("学生id输入错误，请重新输入:");
            student_id = String::new();
        }
    }


}

fn delete_class(class_map: &mut HashMap<String, Class>, theme: &ColorfulTheme) {
    let mut class_id = String::new();

    while class_id.is_empty() {
        class_id = Input::with_theme(theme).with_prompt("请输入要删除的班级id:").interact_text().unwrap();

        if !class_map.contains_key(&class_id) {
            println!("班级id不存在，删除失败");
            class_id = String::new();
        }
    }

    class_map.remove(&class_id);
}

fn class_list(class_map: &HashMap<String, Class>) {
    if class_map.is_empty() {
        println!("班级列表为空，请先添加班级信息");
        println!("==================================");
    }

    for (class_id, class) in class_map {
        println!("班级id:{:?},班级名称:{:?}",class_id,class.name);
    }
}

fn add_class(class_map: &mut HashMap<String, Class>,theme: &ColorfulTheme) {
    let mut class_id:String = String::new();

    while class_id.is_empty() {
        class_id = Input::with_theme(theme).with_prompt("请输入班级id").interact_text().unwrap();

        if class_map.contains_key(&class_id) {
            println!("班级id重复，请重新输入");
            class_id = String::new();
        }
    }

    let mut class_name:String = String::new();
    while class_name.is_empty() {
        class_name = Input::with_theme(theme).with_prompt("请输入班级名称:").interact_text().unwrap();
    }

    let class = Class::new(class_id.clone(),class_name);

    class_map.insert(class_id,class);
}