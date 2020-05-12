enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn route(ip_type: IpAddrKind) {

}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let home = IpAddr2::V4(127,0,0,1);
    println!("Hello, world!");

    let a: Option<String> = Some(String::from("AAA"));


    let x: i8 = 5;
    let y: Option<i8> = Some(6);

    let sum = match y {
        Some(y) => x + y,
        None => x,
    };

    println!("{}", sum);

    println!("{:?}", UsState::Alabama);

    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);
    let none = plus_one(None);
    println!("{:?}", none);


    match 10 {
        1 => println!("1"),
        _ => println!("other"),
    }

    if let Some(q) = Some(100) {
        println!("{}", q);
    }

}
