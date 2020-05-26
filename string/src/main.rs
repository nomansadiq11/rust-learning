fn main() {
    

    let mut a = String::from("Hello World"); 
    a.push_str(" People "); 

    println!("{}", a);


    let m = String::from("Hello Wold"); 
    let m2 = m.clone();
    println!("{}", m2); 
    println!("{}", m); 


}
