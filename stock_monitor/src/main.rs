use clap::{arg, Command};
use yahoo_finance::{history, Interval, Timestamped, Bar};

#[tokio::main]
async fn main() {
    // To call from command line, need to navigate to stock_monitor.exe in target\debug folder and run ".\stock_monitor.exe --ticker AAPL"
    let ticker = get_ticker_from_command_line();    

    // TODO
    //let retrieved_stuff = get_stock_quotes(ticker.to_string());
    //not returning anything yet. only printing from the function directly.
    get_stock_quotes(ticker.to_string()).await;


    // TODO
    // Plot function
    plot_function();
}

fn get_ticker_from_command_line() -> String {
    let matches = Command::new("Stock Market Monitor")
    .version("1.0")
    .about("Takes a stock symbol as input and outputs a chart showing the daily closing price for the last six months")
    .arg(arg!(--ticker <VALUE>).required(true).help("The desired stock ticker symbol to be analyzed and graphed. Ex: AAPL"))
    .get_matches();

    let ticker_arg: &String = matches.get_one::<String>("ticker").expect("required");

    println!("Inputted ticker: {:?}", ticker_arg);

    return ticker_arg.to_string();
}

async fn get_stock_quotes(queried_ticker: String) -> Result<Vec<Bar>, Box<dyn std::error::Error>>{// -> data_type {
    // Use yahoo_finance crate to return stock quotes 
    // https://docs.rs/yahoo-finance/latest/yahoo_finance/ 

    let mut highs: Vec<f64> = Vec::new();
    let mut lows: Vec<f64> = Vec::new();
    let mut opens: Vec<f64> = Vec::new();
    let mut closes: Vec<f64> = Vec::new();
    let mut datetimes: Vec<String> = Vec::new();

    //let data = history::retrieve_interval(&queried_ticker, Interval::_6mo).await.unwrap();
    // Attempt to retrieve stock quotes for the given ticker symbol
    let data = match history::retrieve_interval(&queried_ticker, Interval::_6mo).await {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Could not retrieve desired {} stock quotes from Yahoo", queried_ticker);
            return Err(Box::new(err));
        }
    };

    for bar in &data {
        highs.push(bar.high);
        lows.push(bar.low);
        opens.push(bar.open);
        closes.push(bar.close);
        datetimes.push(bar.datetime().to_string());
    }

    for bar in &data {
        println!("Apple hit an intraday high of ${:.2} on {}.", bar.high, bar.datetime().format("%b %e %Y"));
    }

    Ok(data)
}

fn plot_function(){// stock_data: data_type){
    // Use plotters crate https://docs.rs/plotters/latest/plotters/
    // Show daily closing price for last six months
    // Highlight volatile days, where the stock price varied by more than 2% of the total price (as measured by the difference between the intra-day high and low)
    // Separate function for that?
    // We get extra marks for better plot than just image file
    
    // Finally, the program will print the minimum and maximum closing price for the interval, and the dates on which these values occurred.
    // Separate function?

}