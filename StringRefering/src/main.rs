fn main() {
    let s = String::from("Pakistan");

    let result = length(&s); 

    println!("Lengof the work {}, is {}", s,result)
}


fn length(name: &String) -> usize
{
    name.len()
}