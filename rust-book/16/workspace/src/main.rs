use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];

    println!("{:?}", v);
    let handler = thread::spawn(move || {
        println!("{:?}", v);
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // println!("{:?}", v);
    for i in 1..5 {
        // メインスレッドから数字{}だよ！
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handler.join().unwrap();
}
