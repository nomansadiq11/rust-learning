
use std::io;

fn main() {


    // let mut x = String::new(); 

    // io::stdin().read_line(&mut x).expect("Failed to read line"); 

    
    
    

    // match x 
    // {
    //     1 => println!("User Entered One"),
    //     2 => println!("User Entered Two"),
    //     3 => println!("User Entered Three"),
    //     _ => println!("None"),

    // }


    let mycoin = Coin::Quarter(UsState::Alabama);

    let mycoinvalue  = value_in_cents(mycoin); 



    println!("{:?}", mycoinvalue);


    let four = Some(10); 

    let res = Plus_One(four); 

    println!("{:?}", res); 

}


#[derive(Debug)]
enum Coin{
    Penny, 
    Nickel, 
    Dime, 
    Quarter(UsState),
}


#[derive(Debug)]
enum UsState
{
    Alabama, 
    Alaska, 
}


fn value_in_cents(coin: Coin) -> u8
{
    match coin
    {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }, 
        Coin::Nickel => 5, 
        Coin::Dime=> 10, 
        Coin::Quarter(state) => 
        {
            println!("{:?}", state);
            25
        }
        , 
    }
}



fn Plus_One(x: Option<i32>) -> Option<i32>
{
    match x 
    {
        None => None,
        Some(i) => Some(i + 1)
    }
}