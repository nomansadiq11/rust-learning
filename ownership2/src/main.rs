fn main() {

    let res = give_ownership(); 
    println!("{}", res);

    let s1 = String::from("Pakistan"); 

    let res2 = take_and_giveback(s1); 

    println!("{}", res2)



}



fn give_ownership() -> String
{
    let x = String::from("Hello world");
    x
}

fn take_and_giveback(x:String) -> String
{
    x 
}