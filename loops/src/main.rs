fn main() {

    

    let mut counter = 0; 
    
    
    // let val = loop {

    //     println!("Hello World!"); 

    //     counter = counter + 1; 

    //     if counter == 3
    //     {
    //         break  counter
    //     }
    // }; 
    // println!("{}", val); 


    // let lnum = [1, 2, 3, 4, 5, 6]; 

    // while counter < lnum.len()
    // {
    //     println!("{}", lnum[counter]); 
    //     counter = counter +1; 
    // }


    for a in (0..5).rev()
    {
        println!("{}, Hello world", a); 
    }



    let lnum = [1, 2, 3, 4, 5, 6]; 

    for a in lnum.iter()
    {
        println!("{}, Hello world", a); 
    }


    
    

    

}



fn PrintHelloWorld(x:u32)
{
    let mut counter = 0; 
    loop {

        println!("Hello World!"); 

        counter = counter + 1; 

        if counter == x
        {
            break;
        }
    }


}