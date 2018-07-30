struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a { self.x }
}

fn test() {
    let y = &5;
    let f = Foo {x: y};
    println!("{}", f.x);

    fn foo<'a>(x: &'a i32) {

    }
}

fn main() {
    test()
}
