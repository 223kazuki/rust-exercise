fn main() {
    let v: Vec<i32> = Vec::new();

    let mut v = vec!(1, 2, 3);

    v.push(1);

    let x = 2;

    v.push(x);

    println!("Hello, world! {:?}", v);

    println!("{:?}", v.get(1));
//    println!("{:?}", &v[10]);

    for i in v {
        println!("{}", i);
    }

    let mut v = vec!(1,2,3,4,5,6);
    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);

    let mut s: String = "AAA".to_string();

    s.push_str("_BBB");

    println!("{}", s);

    let s1 = String::from("AAA");
    let s2 = String::from("BBB");
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    println!("{}", s1);
}
