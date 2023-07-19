
fn main() {
    let a = Box::new(vec![1]);
    let b = a.clone();

    let x = Box::into_raw(a);
    let y = Box::into_raw(b);

    println!("{:p}", x);
    println!("{:p}", y);
}