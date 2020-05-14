use std::collections::HashMap;

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

    for c in "あいうえお".chars() {
        println!("{}", c);
    }

    let mut scores = HashMap::new();
    scores.insert("A", 1);
    scores.insert("B", 2);

    println!("{:?}", scores);

    let teams = vec!(String::from("A"), String::from("B"));
    let scores = vec!(1, 2);

    let scores: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();

    println!("{:?}", scores);

    let team = String::from("A");
    let s = scores.get(&team);
    println!("{:?}", s);

    for (k, v) in &scores {
        println!("{:?}, {:?}", k, v);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for t in text.split_whitespace() {
        let count = map.entry(t).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    {
        let v = vec!(1, 2, 4, 2, 6, 9, 1);
        let mut sum = 0;
        for n in &v {
            sum += n;
        }
        let result: f64 = sum as f64 / v.len() as f64;
        println!("{}", result);
    };

    {
        let v = vec!(1, 2, 4, 2, 6, 9, 1, 1, 6);
        let mut m = HashMap::new();
        for i in &v {
            let e = m.entry(i).or_insert(0);
            *e += 1;
        }
        println!("{:?}", m);

        let mut max = 0;
        let mut result = v.get(0);
        for (i, c) in m {
            if (max < c) {
                max = c;
                result = Some(i);
            }
        }
        println!("{:?}", result);

    };
}
