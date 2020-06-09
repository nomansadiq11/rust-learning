fn main() {
    let mut s = String::from("Hello");
    let su = change(&mut s); 
    println!("{}", su); 
}



fn change(name: &mut String)  -> String
{
    name.push_str(", world");
    name
    
}