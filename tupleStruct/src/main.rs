#[derive(Debug)]

struct Color (u32, u32, u32); 
struct Points (u32,u32,u32); 


fn main() {

    let clr = Color(5, 65, 6); 
    let points = Points(10,1,1); 

    

    println!("{:#?}", clr);
    // println!("{:#?}", points);
    


    another_function(clr)
}



fn another_function(x: Color)
{
    println!("{:#?}", x); 
}