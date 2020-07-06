
#![allow(dead_code)]


mod front_house
{
    pub mod hosting
    {
        pub fn addToWaitList()
        {

        }
    }

    pub struct Breakfash
    {
        pub toast:String,
        Seasonal_fruit:String,

    }

    impl Breakfash
    {
        pub fn new(toast:String) -> Breakfash
        {
            Breakfash
            {
                toast, 
                Seasonal_fruit:String::from("graphs"), 

            }
            
        }
    }

    pub enum appetizer
    {
        soup, 
        salad, 
    }

}


    fn _eat_at_resturant()
    {

        // absolute path 
        // crate::front_house::hosting::addToWaitList(); 

        // reletive path
        // super::front_house::hosting::addToWaitList(); 


        let mut meal = front_house::Breakfash::new(String::from("wheat")); 

        meal.toast = String::from("barly"); 

        println!("{}", meal.toast); 


        let soup = front_house::appetizer::soup; 
        let salad = front_house::appetizer::salad; 

    }






mod Server
{
    fn take_order()
    {

    }
    fn order_serve()
    {

    }
    fn payment()
    {

    }
}

