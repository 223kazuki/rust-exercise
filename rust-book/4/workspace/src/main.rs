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

    let s4 = s3;

    println!("{}", s4);

    let s = String::from("FFF");
    let (s2, l) = calculate_length(s);

    println!("{}, {}", s2, l);

    let s = String::from("XXX");
    let l = calculate_length2(&s);
    println!("{}, {}", s, l);


    let mut s = String::from("YYY");
    change(&mut s);

    println!("{}",s);

    let mut s = String::from("Hello");
    let r1 = &mut s;
    change(r1);

    let r3 = &s;
    println!("{}", r3.len());

    let r2 = &mut s;
    change(r2);

    println!("{}", s);

    let s = String::from("aaa");
    let s2 = first_word(&s);

    let s = String::from("hello, world");
    let s2 = &s[1..];

    println!("{}", s2);

    let s = String::from("Hello, world");
    let s2 = first_word(&s);
    println!("{}", s2);

    let s = first_word("AA BB");
    println!("{}", s);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

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


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str("AAA");
}

fn not_dungle(s1: &String) -> &String {
    let s2 = String::from("ZZZ");
    &s1
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
