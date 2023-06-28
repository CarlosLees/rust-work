
//创建一个Rust工程，
// （1）添加一个一层子模块，循环打印从’a’~’Z’ 之间的所有字符
// （2）添加一个二层子模块，循环打印从’A’~’z’ 之间的所有字符
// （3）使用Cargo编译运行此工程

mod submodule1 {
    pub fn print_char() {
        for c in ('Z'..='a').rev() {
            print!("{:?}", c);
        }
        println!()
    }

    pub mod submodule2 {
        pub fn print_char() {
            for i in 'A'..='z' {
                print!("{:?}",i);
            }
        }
    }
}

fn main() {
    submodule1::print_char();
    submodule1::submodule2::print_char();
}
