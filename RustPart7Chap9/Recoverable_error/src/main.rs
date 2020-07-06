use std::io::ErrorKind; 
use std::net::IpAddr; 
use std::fs::File;

fn main() {


    let home : IpAddr = "127.0.01".parse().unwrap();

    // let f = File::open("hello.txt").expect("This file hello not available "); 

    // let f = match f 
    // {
    //     Ok(val) => val, 
    //     Err(Error) => match Error.kind()  {
    //         ErrorKind::NotFound => match File::create("Hello.txt")
    //         {
    //             Ok(f) => f,
    //             Err(verr) => panic!("Unable to create file {}", verr),
    //         }
    //         _ => { panic!("Something went wrong") }
    //     },

    // };

    println!("Hello, world!");
}
