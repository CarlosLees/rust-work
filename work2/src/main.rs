// 自己对所有权、不可变引用、可变引用这三者的规则或特性做一个集中的总结，写一个笔记列表。

// fn main() {
//     let data = vec![1,2,3];
//     let v = 1;
//     if let Some(res) = find_v(data,v) {
//         println!("{}", res);
//     }
// }
//
// fn find_v(data: Vec<i32>,v:i32) -> Option<usize> {
//     for (index,item) in data.iter().enumerate() {
//         if *item == v {
//             return Some(index)
//         }
//     }
//     None
// }

// fn main() {
//     let mut data = vec![1,2,3];
//     for item in data.iter_mut() {
//         data.push(*item + 1);
//     }
// }

fn main() {
    let mut data = vec![1, 2, 3];
    let data1 = vec![&data[0]];
    println!("data[0]: {:p}", &data[0]);

    for i in 0..10000 {
        data.push(i);
    }
    println!("data[0]: {:p}", &data[0]);
}
