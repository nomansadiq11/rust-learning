fn main() {

    let mut s = String::from("foo"); 

    s.push_str(" bar"); 


    s.push('N');

    // println!("{}", s);


    let s1 = String::from("tic"); 
    let s2 =  String::from("tac"); 
    let s3 =  String::from("tok"); 

    let s  = format!("{} {} {}", s1, s2, s3); 


    

    // println!("{}", s); 
}
