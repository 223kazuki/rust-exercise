use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

#[derive(Debug)]
pub struct Guess {
    value: u32
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Error");
        }

        Guess { value }
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    println!("Hello, world!");
    //panic!("panic!");

    let v = vec![1, 2, 3];
    //v[100];

    let file_name = "hello.txt";
    let f = File::open(&file_name);
    let f = match f {
        Ok(f) => f,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            match File::create(&file_name) {
                Ok(f) => f,
                Err(e) => {
                    panic!("Panic!");
                }
            }
        }
        Err(_) => {
            panic!("Error")
        }
    };
    println!("{:?}", f);

    let name = read_user_name_from_file(&file_name).unwrap();

    println!("{}", name);

    let guess = Guess::new(1001);
    println!("{:?}", guess);
}


fn read_user_name_from_file(file_name: &str) -> Result<String, io::Error> {
    let f = File::open(file_name);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

fn read_user_name_from_file2(file_name: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(file_name)?.read_to_string(&mut s)?;
    Ok(s)
}
