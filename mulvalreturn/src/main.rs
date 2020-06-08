fn main() {
    let name = String::from("Hello, world!");
    let (tlen, namne1) = length(name);
    println!("{}", tlen); 
    println!("The length of the word {} is {}", namne1, tlen); 
}



fn length(name : String) -> (usize, String){

    (name.len(), name)
}
