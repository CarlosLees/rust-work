use std::rc::Rc;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use crate::config::application_config::StudentConfig;
use crate::Store;

pub fn student_service(store: Rc<Store>, theme: &ColorfulTheme, student_config: &StudentConfig) {
    let student_map = &store.student_store;
    let club_map = &store.club_store;
    let class_map = &store.class_store;

    println!("目前学生数量:{}", student_map.len());

    let student_select = Select::with_theme(theme).items(&student_config.options).interact().unwrap();

    if !student_map.is_empty() {
        for (id,student) in student_map.iter() {
            print!("学号:{:?},姓名:{:?},",id,student.name);
            if let Some(club_id) = &student.club_id {
                print!("社团:{:?}",club_map.get(club_id.as_str()))
            }

            if let Some(class_id) = &student.class_id {
                print!("班级:{:?}",class_map.get(class_id.as_str()))
            }
            println!();
        }
    }
}