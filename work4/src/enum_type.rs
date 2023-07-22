// 使用枚举包裹三个不同的类型，并放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。

pub struct Cat;

impl Cat {
    pub fn cat_method(&self) {
        println!("fn cat method")
    }
}

pub struct Dog;

impl Dog {
    fn dog_method(&self) {
        println!("fn dog method")
    }
}

pub struct Pig;

impl Pig {
    fn pig_method(&self) {
        println!("fn pig method")
    }
}

pub enum Animal {
    Cat(Cat),
    Dog(Dog),
    Pig(Pig),
}

impl Animal {
    fn cat_method(&self) {
        if let Animal::Cat(cat) = self {
            cat.cat_method();
        }
    }

    fn dog_method(&self) {
        if let Animal::Dog(dog) = self {
            dog.dog_method();
        }
    }

    fn pig_method(&self) {
        if let Animal::Pig(pig) = self {
            pig.pig_method();
        }
    }
}

fn main() {
    let vec = vec![Animal::Cat(Cat),Animal::Dog(Dog),Animal::Pig(Pig)];

    for x in vec {
        x.dog_method();
        x.cat_method();
        x.pig_method();
    }
}


/**
总结

enum 的优点是在编译时就能检查出类型错误，可以提高代码安全性和可维护性。
使用enum可以将不同的类型打包在一起，可以方便地在函数中传递和处理多种类型的值，而不需要使用多个函数重载。

Trait object是一种动态分发类型，可以在运行时确定类型。
优点是允许在运行时动态地选择和替换不同的实现，从而实现灵活的代码结构。
使用 trait object 可以将不同类型的值作为参数传递给函数，从而实现多态性和代码的复用性。

enum 适用于已知类型数量且类型之间有明显区别的场景，trait object适用于类型数量不确定且类型之间有相似性的场景。
enum 在编译时即可确定类型，因此可以获得更好的性能和安全性。trait object 则需要在运行时进行类型检查和转换，因此会带来一定的性能开销。
enum 可以直接访问枚举类型中的值，trait object 则需要进行强制类型转换才能访问其中的值。
*/