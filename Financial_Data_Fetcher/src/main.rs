use std::fs::File;
use std::io::prelude::*;

struct Bitcoin{
    pub price: f64,
}

struct Ethereum{
    pub price:f64,
}

struct SP500{
    pub price:f64,
}


trait Pricing{
    fn fetch_price(&mut self) -> Result<(), String>;

    fn save_to_file(&self) -> Result<(), std::io::Error>;

    fn name(&self) -> &str;
}


impl Pricing for Bitcoin{
    fn fetch_price(&mut self) -> Result<(), String>{
        self.price = 99.99;
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), std::io::Error> {
        let mut file = File::create("bitcoin.txt")?;

    
        let format = format!("The current Bitcoin price is: {}\n", self.price);
        file.write_all(format.as_bytes())?;

        Ok(())
    }

    fn name (&self) -> &str {
        "Bitcoin"
    }
}



impl Pricing for Ethereum{
    fn fetch_price(&mut self) -> Result<(), String>{
        self.price = 99.99;
        Ok(())
    }

    fn save_to_file(&self) -> Result<(), std::io::Error> {
        let mut file = File::create("ethereum.txt")?;

    
        let format = format!("The current Ethereum price is: {}\n", self.price);
        file.write_all(format.as_bytes())?;

        Ok(())
    }

    fn name (&self) -> &str {
        "Ethereum"
    }
}



impl Pricing for SP500{
    fn fetch_price(&mut self) -> Result<(), String>{
        self.price = 99.99;
        Ok(())
    }


    fn save_to_file(&self) -> Result<(), std::io::Error> {
        let mut file = File::create("sp500.txt")?;

    
        let format = format!("The current SP500 price is: {}\n", self.price);
        file.write_all(format.as_bytes())?;

        Ok(())
    }

    fn name (&self) -> &str {
        "SP500"
    }
}


fn main() {


    let mut values: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin{
             price: 1234.44 
        }),
        Box::new(Ethereum{
            price: 14.901 
        }),
         
        Box::new(SP500{
            price: 15.332 
        }),
    ];

    for value in values.iter_mut() {
        match value.fetch_price(){
            Ok(_) => println!("{} Fetched!", value.name()),
            Err(_) =>println!("Unable to fetch {}!", value.name()),
        };
        match value.save_to_file(){
            Ok(_) => println!("{} Saved!\n", value.name()),
            Err(_) =>println!("Unable to save {}!", value.name()),
        };
    }

    
}
