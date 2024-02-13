use clap::{arg, Command};
use plotters::prelude::*;
use yahoo_finance::{history, Interval, Timestamped, Bar};

#[tokio::main]
async fn main() {
    let ticker = get_ticker_from_command_line();    

    if let Ok(stock_data) = get_stock_quotes(ticker.to_string()).await {
        match plot_function(stock_data) {
            Ok(_) => println!("Plot generated successfully."),
            Err(e) => eprintln!("Failed to generate plot: {}", e),
        }
    } else {
        eprintln!("Failed to retrieve stock data.");
    }
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

    // stock_data: data_type){
    // Use plotters crate https://docs.rs/plotters/latest/plotters/
    // Show daily closing price for last six months
    // Highlight volatile days, where the stock price varied by more than 2% of the total price (as measured by the difference between the intra-day high and low)
    // Separate function for that?
    // We get extra marks for better plot than just image file
    
    // Finally, the program will print the minimum and maximum closing price for the interval, and the dates on which these values occurred.
    // Separate function?

    fn plot_function(stock_data: Vec<Bar>) -> Result<(), Box<dyn std::error::Error>> {
        // Define the path to the output image
        let root_area = BitMapBackend::new("stock_chart.png", (1280, 720)).into_drawing_area();
        
        root_area.fill(&WHITE)?;

        let min_date = stock_data.first().unwrap().datetime();
        let max_date = stock_data.last().unwrap().datetime();
        let max_price = stock_data.iter().map(|x| x.high).fold(0./0., f64::max);
        let min_price = stock_data.iter().map(|x| x.low).fold(0./0., f64::min);

        let mut chart = ChartBuilder::on(&root_area)
            .caption("Stock Price Over Last Six Months", ("sans-serif", 50).into_font())
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(min_date..max_date, min_price..max_price)?;

        chart.configure_mesh().draw()?;

        // Plotting the closing prices
        chart.draw_series(LineSeries::new(
            stock_data.iter().map(|bar| (bar.datetime(), bar.close)),
            &RED,
        ))?;

        // Highlighting volatile days
        for bar in stock_data.iter().filter(|bar| (bar.high - bar.low) / bar.close > 0.02) {
            chart.draw_series(PointSeries::of_element(
                [(bar.datetime(), bar.close)],
                5,
                &BLUE,
                &|coord, size, style| {
                    EmptyElement::at(coord) + Circle::new((0, 0), size, style.filled())
                },
            ))?;
        }

        println!("Minimum Closing Price: {:.2}", min_price);
        println!("Maximum Closing Price: {:.2}", max_price);

        Ok(())
    }
