use clap::{arg, Command};

fn main() {
    // To call from command line, need to navigate to stock_monitor.exe target folder and run ".\stock_monitor.exe --one words --two morewords"
    // Using https://docs.rs/clap/latest/clap/_tutorial/chapter_1/index.html as reference
    // TODO: adjust to single argument, implement --help, maybe functionalize this input stuff
    let matches = Command::new("Stock Market Monitor")
        .version("1.0")
        .about("Fill description later")
        .arg(arg!(--two <VALUE>).required(true))
        .arg(arg!(--one <VALUE>).required(true))
        .get_matches();

    println!(
        "two: {:?}",
        matches.get_one::<String>("two").expect("required")
    );
    println!(
        "one: {:?}",
        matches.get_one::<String>("one").expect("required")
    );

    
    let ticker = matches.get_one::<String>("one").expect("required");

    println!("Inputted ticker: {}", ticker);

    // TODO
    let retrieved_stuff = get_stock_quotes(ticker.to_string());

    // TODO
    // Plot function
    plot_function();
}

fn get_stock_quotes(queried_ticker: String) {// -> data_type {
    // Use yahoo_finance crate to return stock quotes 
    // https://docs.rs/yahoo-finance/latest/yahoo_finance/ looks kinda old (3 years no updates). I found yahoo_finance_api which is more recent, but haven't researched
    // Make sure to handle bad stock symbol
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