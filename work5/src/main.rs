// 一个简单的声明宏,并理解其代码结构，和编译过程。

#[macro_export]
macro_rules! my_vec {
    () => {
        std::vec::Vec::new()
    };
    ($($el:expr),*) => ({
        let mut v = std::vec::Vec::new();
        $(v.push($el);)*
        v
    });
    ($el:expr;$len:expr) => {
        std::vec::from_elem($el, $len)
    }
}

#[macro_export]
macro_rules! hash_map {
    () => {
        std::collections::HashMap::new()
    };
    ($([$el_key:expr,$el_value:expr]),*) => ({
        let mut map = std::collections::HashMap::new();
        $(map.insert($el_key,$el_value);)*
        map
    })
}

fn main() {
    let v = my_vec!{1,2,3};
    println!("v:{:?}", v);

    let v1 = my_vec!(1;10);
    println!("v1:{:?}", v1);

    let map = hash_map!([1,2],[3,4]);
    println!("map:{:?}", map);
}
