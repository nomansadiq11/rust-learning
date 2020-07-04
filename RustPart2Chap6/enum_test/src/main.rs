fn main() {
    
    let fout = IPAddressKind::V4(String::from("127.0.0.1")); 
    let six = IPAddressKind::V6(String::from("::1")); 
    
    let IP1 = IPAddress
    {
        Kind: IPAddressKind::V4(String::from("127.0.0.1")), 
        address: String::from("127.0.0.1"), 
    };
    
    println!("{:?}", fout);
    println!("{:?}", six);

    println!("{:?}", IP1); 
    


    let msg1 = Message::Quite;
    let msg2  =  Message::Write(String::from("Hello World")); 
    let msg3  = Message::Move{x:10, y:10}; 
    let msg4 = Message::ChangeColor(10, 10, 10); 

    println!("{:?}", msg1);
    println!("{:?}", msg2);
    println!("{:?}", msg3);
    println!("{:?}", msg4);

    let msg = Message::Write(String::from("Hello World")); 
    msg.call(); 


    let some_num = Some(10); 

    println!("{:?}", some_num); 

    let some_string = Some(String::from("HI from SOME string ")); 

    println!("{:?}", some_string); 


    let some_none : Option<i32> = None; 

    println!("{:?}", some_none); 


    // let x:i8 = 5; 

    // let y: Option<i8> = None; 

    // let sum = x + y; 

    // println!("The sume value {}", sum); 


    


}


#[derive(Debug)]
enum IPAddressKind{
    V4(String), 
    V6(String),
}



#[derive(Debug)]
struct IPAddress
{
    Kind : IPAddressKind,
    address:String


}

#[derive(Debug)]
enum Message{
    Quite, 
    Write(String),
    Move{x:u32, y:u32}, 
    ChangeColor(u32, u32, u32), 
}


impl Message
{
    fn call(&self) 
    {
        println!("{:?}", self); 
    }
}