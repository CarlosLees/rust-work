// 定义三个不同的类型，使用Trait Object，将其放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。

trait Animal {
    fn eat(&self);
}

pub struct Cat;

impl Animal for Cat {
    fn eat(&self) {
        println!("fn cat eat")
    }
}

pub struct Dog;

impl Animal for Dog {
    fn eat(&self) {
        println!("fn dog eat")
    }
}

pub struct Pig;

impl Animal for Pig {
    fn eat(&self) {
        println!("fn pig eat")
    }
}

fn main() {
    let vec:Vec<&dyn Animal> = vec![&Cat, &Dog, &Pig];

    for x in vec {
        x.eat();
    }

}

/**
总结

enum 的优点是在编译时就能检查出类型错误，可以提高代码安全性和可维护性。
使用 enum 可以将不同的类型打包在一起，可以方便地在函数中传递和处理多种类型的值，而不需要使用多个函数重载。

Trait object 是一种动态分发类型，可以在运行时确定类型。
优点是允许在运行时动态地选择和替换不同的实现，从而实现灵活的代码结构。
使用 trait object 可以将不同类型的值作为参数传递给函数，从而实现多态性和代码的复用性。

enum 适用于已知类型数量且类型之间有明显区别的场景，而 trait object 则适用于类型数量不确定且类型之间有相似性的场景。
enum 在编译时即可确定类型，因此可以获得更好的性能和安全性。而 trait object 则需要在运行时进行类型检查和转换，因此会带来一定的性能开销。
enum 可以直接访问枚举类型中的值，而 trait object 则需要进行强制类型转换才能访问其中的值。
 */