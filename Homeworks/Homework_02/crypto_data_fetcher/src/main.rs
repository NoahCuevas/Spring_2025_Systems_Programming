use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;
use std::thread::sleep;
use serde::{Serialize, Deserialize};
use ureq::serde_json;

trait Pricing { // for fetching & saving prices
    fn fetch_price(&mut self);
    fn save_to_file(&self);
}

#[derive(Debug, Serialize, Deserialize)]
struct Currency {
    usd: f64,
}

//----------------------------------------------

#[derive(Debug, Serialize, Deserialize)]// bitcoin struct
struct BitcoinResponse {
    bitcoin: Currency,
}
#[derive(Debug)]
struct Bitcoin {
    price: Option<f64>,
}
impl Bitcoin {
    fn new() -> Self {
        Bitcoin { price: None }
    }
}

impl Pricing for Bitcoin {
    fn fetch_price(&mut self) {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        match ureq::get(url).call() {
            Ok(response) => match response.into_json::<BitcoinResponse>() {
                Ok(res) => {
                    if let Err(e) = std::fs::write("bitcoin.json", serde_json::to_string_pretty(&res).unwrap()) {
                        eprintln!("Failed to write file: {}", e);
                    }
                }
                Err(e) => eprintln!("Failed to parse JSON: {}", e),
            },
            Err(e) => eprintln!("Request failed: {}", e),
        }
    }

    fn save_to_file(&self) {
        if let Some(price) = self.price {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("bitcoin.txt")
                .expect("Unable to open or create file.");
            let line = format!("Bitcoin: ${:.2}\n", price);
            file.write_all(line.as_bytes()).expect("Failed to write to file.");
        }
    }
}

// copied and edited the bicoin code for etherium
#[derive(Debug, Serialize, Deserialize)]
struct EthereumResponse {
    ethereum: Currency,
}
#[derive(Debug)]
struct Ethereum {
    price: Option<f64>,
}
impl Ethereum {
    fn new() -> Self {
        Ethereum { price: None }
    }
}

impl Pricing for Ethereum {
    fn fetch_price(&mut self) {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        match ureq::get(url).call() {
            Ok(response) => match response.into_json::<EthereumResponse>() {
                Ok(res) => {
                    if let Err(e) = std::fs::write("ethereum.json", serde_json::to_string_pretty(&res).unwrap()) {
                        eprintln!("Failed to write file: {}", e);
                    }
                }
                Err(e) => eprintln!("Failed to parse JSON: {}", e),
            },
            Err(e) => eprintln!("Request failed: {}", e),
        }
    }

    fn save_to_file(&self) {
        if let Some(price) = self.price {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("ethereum.txt")
                .expect("Unable to open/create file.");
            let line = format!("Ethereum: ${:.2}\n", price);
            file.write_all(line.as_bytes()).expect("Couldn't write to the file.");
        }
    }
}

// again, I copied etheriums code for sp500
#[derive(Debug, Serialize, Deserialize)]
struct SP500Response {
    symbol: String,
    price: f64,
}
#[derive(Debug)]
struct SP500 {
    price: Option<f64>,
}
impl SP500 {
    fn new() -> Self {
        SP500 { price: None }
    }
}

impl Pricing for SP500 {
    fn fetch_price(&mut self) {
        // didn't realize that sp500 wasn't a crypto currency so we have to use a different website.
        // but other than that the code should be the same

        // nevermind this is one is such a pain in the butt
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m";
        match ureq::get(url).call() {
            Ok(response) => match response.into_json::<Vec<SP500Response>>() {
                Ok(res) => {
                    if let Some(sp500) = res.into_iter().find(|entry| entry.symbol == "^GSPC") {
                        if let Err(e) = std::fs::write("sp500.json", serde_json::to_string_pretty(&sp500).unwrap()) {
                            eprintln!("Failed to write file: {}", e);
                        }
                    } else {
                        eprintln!("Couldn't find S&P 500 data.");
                    }
                }
                Err(e) => eprintln!("Failed to parse JSON: {}", e),
            },
            Err(e) => eprintln!("Request failed: {}", e),
        }
    }

    fn save_to_file(&self) {
        if let Some(price) = self.price {
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("sp500.txt")
                .expect("Unable to open or create file.");
            let line = format!("S&P 500: ${:.2}\n", price);
            file.write_all(line.as_bytes()).expect("Failed to write to file.");
        }
    }
}


fn main() {
    let mut assets: Vec<Box<dyn Pricing>> = vec![
        Box::new(Bitcoin::new()),
        Box::new(Ethereum::new()),
        Box::new(SP500::new()),
    ];

    loop {
        for asset in assets.iter_mut() {
            asset.fetch_price();
            asset.save_to_file();
        }
        println!("Waiting for 10 seconds...\n");
        sleep(Duration::from_secs(10));
    }
}

