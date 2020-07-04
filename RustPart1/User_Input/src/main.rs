use std::io; 

fn main() {

    let mut s = String::new(); 

    io::stdin().read_line(&mut s).expect("Failed to read line"); 
    


    let mut sint:u32 = s.trim().parse().expect("Please enter a digigt"); 

    sint = sint + 1; 

    println!("{}", sint);

}
