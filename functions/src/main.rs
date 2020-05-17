fn main() {
    

    
    let num = {
        let i = 19; 
        i +1
    }; 
    
    let (val, val1) = sequre(2, 3.5); 
    
    println!("{}, {}", val, val1); 

}


fn CakeMake()
{
    println!("1. Milk"); 
    println!("1. Egg"); 
}



fn sequre(x:u32, y:f64) -> (u32, f64)
{
    let val = x * x; 
    let val1 = y * y; 
    (val, val1)

}