fn main() {
    
    let s  = String::from("Hello, world!");

    take_ownership(s);

    
    let itval = 10; 
    make_copy(itval); 

    println!("{}", itval); 

}



fn take_ownership(x: String)
{
    println!("{}", x); 
}



fn make_copy(x:i32)
{
    println!("{}", x);
}