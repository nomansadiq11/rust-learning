
#[derive(Debug)]

struct Book
{
    name:String, 
    author : String, 
    price: u16, 
    available: bool,
}


fn main() {
    

    let bool_1 = Book
    {
        name:String::from("Book 1"), 
        author:String::from("Noman"),
        price:500,
        available:true,
    };

    let bool_2 = Book
    {
        name:String::from("Book 2"), 
        author:String::from("sadiq"),
        price:50,
        available:false,
    };


    println!("{:#?}", bool_1); 
    println!("{:#?}", bool_2); 
    

}
