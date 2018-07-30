fn main() {
    let (x, _y) = (1, 2);
    let _a: i32 = 5;
    let mut _b = 5;
    _b = 10;
    {
        println!("{}", x);
        let x = 12;
        println!("{}", x);
    }
    let f = add_one;
    print_number(f(x));

    let c = [1, 2, 3];
    print_number(c[0]);
    println!("{}", &c[1..][0]);
}

fn print_number(x: i32) {
    println!("{}", x);
}

fn add_one(x: i32) -> i32 {
    x + 1
}
