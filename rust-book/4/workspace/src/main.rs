fn main() {
    {
        let s = "hello";
    }

    let mut s = String::from("Hello");
    s.push_str(", world!");

    println!("{}", s);

    let x = 5;
    let y = x;

    println!("{}", x);

    let mut x = 10;
    let mut y = 20;
    y = x;

    println!("{}", x);

    let s1 = String::from("AAA");
    let s2 = s1;

    println!("{}", s2);

    let s1 = String::from("BBB");
    let s2 = s1.clone();

    println!("{}", s1);
    println!("{}", s2);

    let s = String::from("CCC");
    takes_ownership(s);
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();

    println!("{}", s1);

    let s2 = String::from("EEE");
    let s3 = takes_and_gives_back(s2);

    println!("{}", s3);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integerr: i32) {
    println!("{}", some_integerr);
}

fn gives_ownership() -> String {
    let some_string = String::from("DDD");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
