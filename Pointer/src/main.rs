fn main() {

    let a:u8 = 10; 

    let b = &a;
    let c = &b; 

    println!("a:{}, b{}, c{}", a, b, c); 

    println!("The addres of the a is {:b}", a); 
    println!("The addres of the b is {:b}", b); 



}
