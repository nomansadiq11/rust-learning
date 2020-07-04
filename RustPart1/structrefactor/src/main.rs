#[derive(Debug)]

struct Rectangle
{
    height:u32,
    width:u32

}

fn main() {

    let height = 100; 
    let width = 50; 


    let dim  =(100, 50); 

    let rec = Rectangle
    {
        height:100,
        width:50
    };


    // println!("The area : {}", area(height, width));

    // println!("The area : {}", area2(dim));

    println!("The are {}", area3(&rec));

    println!("{:#?}", area3(&rec));

}


fn area(height:u32, width:u32) -> u32
{
    height * width
}

fn area2(dim:(u32, u32)) -> u32
{
    dim.0 * dim.1

}

fn area3(rec: &Rectangle) ->u32
{
    rec.height * rec.width
}