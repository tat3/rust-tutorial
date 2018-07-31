
// Definition of sample trait
trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

// Static dispatch using trait
fn do_something_static<T: Foo>(x: T) {
    println!("Static!");
    x.method();
}

fn do_something_static2<T: Foo>(x: &T) {
    println!("Static 2!");
    x.method();
}

// Dynamic dispatch using trait object
fn do_something_dynamic(x: &Foo) {
    println!("Dynamic!");
    x.method();
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();
    let z = "Hello".to_string();

    do_something_static(x);
    do_something_static2(&y);
    do_something_static(y);
    do_something_dynamic(&x as &Foo);
    do_something_dynamic(&z);
}
