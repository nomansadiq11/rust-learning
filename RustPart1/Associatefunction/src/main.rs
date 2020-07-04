#[derive(Debug)]
struct Rectangle
{
    height:u32, 
    width:u32,
}

impl Rectangle
{
    fn sequre(size: u32) -> Rectangle
    {
        Rectangle
        {
            height:size,
            width:size
        }
    }
}

fn main() {

    let res = Rectangle::sequre(8); 

    println!("sequre is {:#?}", res);
}
