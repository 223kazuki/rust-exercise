use std::mem::drop;
use std::ops::Deref;
use std::rc::Rc;
use List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
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

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Rc: {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("Rc: {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("Rc: {}", Rc::strong_count(&a));
    }
    println!("Rc: {}", Rc::strong_count(&a));
}
