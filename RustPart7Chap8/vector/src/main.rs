fn main() {


    let v: Vec<i32> = Vec::new(); 

    let mut v1 = vec![10, 20, 30, 40];

    println!("{:?}", v1);

    v1.push(50); 


    let ele1 = v1[1]; 
    let ele = v1.get(1); 


    

    
    // println!("{:?}", ele1);
    // println!("{:?}", ele);


    // match ele 
    // {
    //     Some(val) => println!("{}", val),
    //     None => println!("None Match"),
    // }



    // for i in &mut v1
    // {
    //     *i += 50;
    //     println!("{}", i)
    // }



    let row = vec![SpreadSheetCell::Int(50), SpreadSheetCell::float(10.2), SpreadSheetCell::Text(String::from("Hello World"))];


    println!("{:?}", row); 





}


#[derive(Debug)]
enum SpreadSheetCell
{
    Int(i32), 
    float(f32),
    Text(String)
}