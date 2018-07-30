fn foo1() {
    let v = vec![1, 2, 3];
    let _v2 = v;
    // println!("v[0] is: {}", v[0]);
}

fn foo2(_v1: &Vec<i32>, _v2: &Vec<i32>) -> i32 {
    42
}

fn foo3() {
    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
        println!("{}, {}", y, *y);
    }
    println!("{}", x);
}

fn foo4() {
    let mut v = vec![1, 2, 3];

    for i in &v {
        println!("{}", i);
    }
}


fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let _ans = foo2(&v1, &v2);
    println!("{}", v1[0]);
    foo1();
    foo3();
    foo4();
}
