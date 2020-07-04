fn main() {

    // let mut s = String::from("Hello "); 

    // {
    //     let a = &mut s;
    //     a.push_str("World"); 
    //     println!("{}", a); 
    // }
    // {
    //     let b = &mut s; 
    //     println!("{}", b); 
    // }
    
    
    Immutable(); 

    

    // mut cannot be two pointer ref 
}


fn Immutable()
{
    let s = String::from("Hello"); 
    let b = &s; 
    let c = &s;
    let d = &s;

    println!("{}, {}, {}", b, c, d); 

}