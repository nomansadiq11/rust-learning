
struct Rectangle
{
    height:u32,
    width:u32
}


impl Rectangle
{
    fn area (&self) -> u32
    {
        self.width * self.height
    }
    
}


impl Rectangle
{
    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.height > other.height && self.width > other.width
    }
}

fn main() {

    let rec1 = Rectangle { height:100, width:50 };
    let rec2 = Rectangle { height:90, width:40 };
    let rec3 = Rectangle { height:80, width:30 };

    let res = rec1.can_hold(&rec2); 
    let res2 = rec2.can_hold(&rec1); 

    println!("rec 1 can hold rec2 ? {}", res);

    println!("rec 2 can hold rec1 ? {}", res2);
}
