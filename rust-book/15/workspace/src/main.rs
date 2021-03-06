use std::cell::RefCell;
use std::mem::drop;
use std::ops::Deref;
use std::rc::Rc;
use List::{Cons, Nil};

enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomStartPointer {
    data: String,
}

impl Drop for CustomStartPointer {
    fn drop(&mut self) {
        println!("Drop!");
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            // 警告: 割り当ての75％以上を使用してしまいました
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            // 切迫した警告: 割り当ての90%以上を使用してしまいました
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            // エラー: 割り当てを超えています
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

fn main() {
    let b = Box::new(5);
    println!("{}", b);

    let x = 5;
    let y = MyBox::new(x);

    println!("{}", *y);

    let c = CustomStartPointer {
        data: String::from("my stuff"),
    };
    {
        let d = CustomStartPointer {
            data: String::from("other stuff"),
        };
    }
    drop(c);

    println!("CustomStartPointers created.");

    let valus = Rc::new (RefCell::new (5))
}
