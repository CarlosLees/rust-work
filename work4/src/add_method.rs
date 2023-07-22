//搜索相关文档，为你自己定义的一个类型或多个类型实现加法运算（用符号 +），并构思使用Trait Object实现类型方法的调用。

use std::ops::Add;

#[derive(Debug)]
struct Rectangle {
    side: u32,
    wide: u32
}

impl Rectangle {
    fn new(side:u32,wide: u32) -> Self {
        Self {
            side,
            wide,
        }
    }
}

impl Add for Rectangle {
    type Output = Rectangle;

    fn add(self, rhs: Self) -> Self::Output {
        Rectangle {
            side: self.side + rhs.side,
            wide: self.wide + rhs.wide,
        }
    }
}

trait Area {
    fn area(&self) -> u32;
}

impl Area for Rectangle {
    fn area(&self) -> u32 {
        self.side * self.wide
    }
}

fn main() {
    let rectangle1 = Rectangle::new(1,2);
    let rectangle2 = Rectangle::new(3,4);

    let rectangle_add_res = rectangle1 + rectangle2;
    println!("{:?}",rectangle_add_res);

    let v:&dyn Area = &rectangle_add_res;

    println!("{:?}",v.area());
    println!("{}", rectangle_add_res.area());
}