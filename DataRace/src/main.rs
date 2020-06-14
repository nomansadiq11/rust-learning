fn main() {

    let mut s = String::from("Hello World"); 

    {
        let a = &mut s;
        println!("{}", a); 
    }
    {
        let b = &mut s; 
        println!("{}", b); 
    }
    
    

    

    // mut cannot be two pointer ref 
}
