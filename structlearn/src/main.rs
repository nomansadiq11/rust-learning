
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

    let mut bool_2 = Book
    {
        name:String::from("Book 2"), 
        author:String::from("sadiq"),
        price:50,
        available:false,
    };


    bool_2.name = String::from("Book 2 update"); 

    let book_3 = build(String::from("Hello"), String::from("author")); 

    println!("{:#?}", bool_1); 
    println!("{:#?}", bool_2); 
    println!("{:#?}", book_3); 
    

}



fn build (name:String, author:String) -> Book
{

    Book
    
    {
        name, 
        author,
        price:50,
        available:false,
    }

}