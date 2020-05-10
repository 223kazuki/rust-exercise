fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    let mut y = 6;
    println!("The value of y is: {}", y);
    y = 7;
    println!("The value of y was changed to: {}", y);

    let _guess: u32 = "42".parse().expect("Not a number!");
    let a: isize = 1;

    println!("{}", a);

    let b = 100_000_000;
    println!("{}", b);

    let c = a * 30 / 20 + 4 - 10;
    println!("{}", c);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("{}", y);
    println!("{}", tup.2);

    let a = [1, 2, 3, 4];
    println!("{}", a[1]);

    let r = another_function(10, 20);
    println!("Returned!: {}", r);

    // hello, world

    let num = 10;
    let a = if num < 10 {
        println!("Match!");
        10
    } else if num < 5 {
        println!("Unmatch!");
        20
    } else {
        println!("Else!");
        100
    };

    println!("If result: {}", a);

    let b = if true {10} else {20};

    println!("If result: {}", b);

    let mut i = 0;
    while i < 10 {
        println!("While! {}", i);
        i = i + 1;
    }

    let a = [1, 2, 3];
    for e in a.iter() {
        println!("For: {}", e);
    }

    let it = (1..10);
    for e in it.rev() {
        println!("Reversed: {}", e);
    }
}

fn another_function(x: i32, y: i32) -> i32{
    println!("Function call! with {}, {}", x, y);
    let z = x + y;
    println!("Calculated! {}", z);
    let a = {
        let b = 100;
        b * 2
    };
    println!("Calculated! {}", a);
    {
        let r = 100;
        r
    }

}
