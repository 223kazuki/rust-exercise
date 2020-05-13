extern crate communicator;

mod outermost {
    pub fn middle_function() {
        middle_secret_function();
    }

    fn middle_secret_function() {
        println!("!!!");
    }

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
        pub enum TrafficLight {
            Red,
            Yellow,
            Green,
        }
    }
}

use a::series::of::nested_modules;
use a::series::TrafficLight::*;

fn main() {
    communicator::client::connect();
    println!("Hello!");

    outermost::middle_function();
//    outermost::middle_secret_function();
  //  outermost::inside::inner_function();
    //outermost::inside::secret_function();

    nested_modules();

    let red = Red;
}
