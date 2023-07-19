use std::collections::HashMap;
use dialoguer::{Input, Select};
use dialoguer::theme::ColorfulTheme;
use crate::config::application_config::StudentConfig;
use crate::entity::class::Class;
use crate::entity::club::Club;
use crate::entity::student::Student;
use crate::Store;

pub fn student_service(store: &mut Store, theme: &ColorfulTheme, student_config: &StudentConfig) {

    loop {
        let student_select = Select::with_theme(theme).items(&student_config.options).interact().unwrap();

        match student_select {
            0 => {
                student_list(&store.student_store,&store.club_store,&store.class_store);
            },
            1 => {
                add_student(theme, &mut store.student_store);
            },
            2 => {
                update_student(&mut store.student_store,theme);
            },
            3 => {
                delete_student(&mut store.student_store,theme);
            },
            4 | _ => {
                break
            }
        }

    }
}

fn student_list(student_map:&HashMap<String,Student>,club_map: &HashMap<String,Club>,class_map: &HashMap<String,Class>) {
    if !student_map.is_empty() {
        for (id,student) in student_map.iter() {
            print!("学号:{:?},姓名:{:?},年龄:{:?}",id,student.name,student.age);
            if let Some(club_id) = &student.club_id {
                print!(",社团:{:?}",club_map.get(club_id.as_str()))
            }

            if let Some(class_id) = &student.class_id {
                print!(",班级:{:?}",class_map.get(class_id.as_str()))
            }
            println!("\n ==========================================");
        }
    }else {
        println!("当前学生数量为0");
        println!("============================================");
    }
}

fn add_student(theme: &ColorfulTheme,student_map: &mut HashMap<String, Student>) {
    let mut id:String = String::new();
    let mut name:String = String::new();
    let mut age = 0;

    while id.is_empty(){
        id = Input::with_theme(theme).with_prompt("请输入学生id:").interact_text().unwrap();
        if student_map.contains_key(&id) {
            println!("学生id重复，请重新添加");
            id = String::new();
        }
    }

    while name.is_empty() {
        name = Input::with_theme(theme).with_prompt("请输入学生姓名:").interact_text().unwrap();
    }

    while age == 0 {
        age = Input::with_theme(theme).with_prompt("请输入学生年龄:").interact_text().unwrap();
    }

    let student = Student::new(id.clone(), name, age);
    student_map.insert(id, student);
}


fn update_student(student_map: &mut HashMap<String,Student>,theme: &ColorfulTheme) {
    let mut id = String::new();

    while id.is_empty() {
        id = Input::with_theme(theme).with_prompt("请输入要进行修改的学生id:").interact_text().unwrap();

        if !student_map.contains_key(&id) {
            id = String::new();
            println!("输入的学生id不存在该学生，请重新输入");
        }
    }

    let change_options = vec!["姓名","年龄","退出"];
    let select = Select::with_theme(theme).with_prompt("请选择要修改的内容：").items(&change_options).interact().unwrap();

    match select {
        0 => {
            //修改姓名
            let name:String = Input::with_theme(theme).with_prompt("请输入新的姓名:").interact_text().unwrap();
            let mut student = student_map.get(&id).unwrap().clone();
            student.update_name(name);
            student_map.insert(id,student.clone());
        },
        1 => {
            //修改年龄
            let age:i32 = Input::with_theme(theme).with_prompt("请输入新的年龄:").interact_text().unwrap();
            let mut student = student_map.get(&id).unwrap().clone();
            student.update_age(age);
            student_map.insert(id,student.clone());
        },
        3 | _ => {
            return;
        }
    }
}

fn delete_student(student_map: &mut HashMap<String, Student>, theme: &ColorfulTheme) {
    let mut id = String::new();

    while id.is_empty() {
        id = Input::with_theme(theme).with_prompt("请输入要进行删除的学生id:").interact_text().unwrap();

        if !student_map.contains_key(&id) {
            id = String::new();
            println!("输入的学生id不存在该学生，请重新输入");
        }
    }

    student_map.remove(&id).unwrap();
}