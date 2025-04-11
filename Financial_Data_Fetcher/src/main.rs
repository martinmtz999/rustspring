// This section importes the libraries needed 
use std::fs::File;
use std::io::prelude::*;
use serde::Deserialize;


// Defined the 3 structs for each financial asset
struct Bitcoin{
    pub price: f64,
}

struct Ethereum{
    pub price:f64,
}

struct SP500{
    pub price:f64,
}// Defined the structs needed to deserialize the json response from the api for bitcoing/ethereum
#[derive(Deserialize)]
struct AssetP {
    usd: f64,
}

#[derive(Deserialize)]
struct CgResponse {
    bitcoin: Option<AssetP>,
    ethereum: Option<AssetP>,
}

// Defined the structs needed to deserialize the json response from the api for the sp500
#[derive(Deserialize)]
struct YahooRoot {
    chart: Chart,
}

#[derive(Deserialize)]
struct Chart {
    result: Vec<ChartResult>,
}

#[derive(Deserialize)]
struct ChartResult {
    meta: Meta,
}

#[derive(Deserialize)]
struct Meta {
    regularMarketPrice: f64,
}


// Defined the trait which each struct shares
trait Pricing{
    fn fetch_price(&mut self) -> Result<(), String>;

    fn save_to_file(&self) -> Result<(), std::io::Error>;

    fn name(&self) -> &str;
}

// The implementation for the methods needed to fetch and write to the text file for Bitcoin
impl Pricing for Bitcoin{
    fn fetch_price(&mut self) -> Result<(), String> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        let response = ureq::get(url).call().map_err(|e| e.to_string())?;
    
        let j_parse: CgResponse = response.into_json().map_err(|e| e.to_string())?;
    
        if let Some(data) = j_parse.bitcoin {
            self.price = data.usd;
            Ok(())
        } else {
            Err("Bitcoin price not found".to_string())
        }
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

// The implementation for the methods needed to fetch and write to the text file for Ethereum
impl Pricing for Ethereum{
    fn fetch_price(&mut self) -> Result<(), String> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        let response = ureq::get(url).call().map_err(|e| e.to_string())?;
    
        let j_parse: CgResponse = response.into_json().map_err(|e| e.to_string())?;
    
        if let Some(data) = j_parse.ethereum {
            self.price = data.usd;
            Ok(())
        } else {
            Err("Ethereum price not found".to_string())
        }
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


// The implementation for the methods needed to fetch and write to the text file for SP500
impl Pricing for SP500{
    fn fetch_price(&mut self) -> Result<(), String> {
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC";
        let response = ureq::get(url).call().map_err(|e| e.to_string())?;

        let j_parse: YahooRoot = response.into_json().map_err(|e| e.to_string())?;

        let price = j_parse.chart.result
            .get(0)
            .ok_or("error")?
            .meta
            .regularMarketPrice;

        self.price = price;
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

// This creates a vector of the 3 financial assets, in order to loop through them
    let mut values: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin{
             price: 0.0
        }),
        Box::new(Ethereum{
            price: 0.0
        }),
         
        Box::new(SP500{
            price: 0.0
        }),
    ];

// This loops through the vector created above, loops through each element in an endless loop that executes every 10 seconds
    loop{
        for value in values.iter_mut() {
        // this uses the methods from the impl for each asset to retrieve and save the price, and handles errors
            match value.fetch_price(){
                Ok(_) => println!("{} Fetched!", value.name()),
                Err(_) =>println!("Unable to fetch {}!", value.name()),
            };
            match value.save_to_file(){
                Ok(_) => println!("{} Saved!\n", value.name()),
                Err(_) =>println!("Unable to save {}!", value.name()),
            };
        }
        // this allows the loop to execute 
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
    
}
