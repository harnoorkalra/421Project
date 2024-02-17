use clap::{arg, Command};
use plotters::prelude::*;
use yahoo_finance::{history, Interval, Timestamped, Bar};
use chrono::{TimeZone, Utc};

#[tokio::main]
async fn main() {
    // Get the ticker symbol from the command line arguments
    let ticker = get_ticker_from_command_line();    

    // Attempt to retrieve stock quotes for the given ticker symbol
    if let Ok(stock_data) = get_stock_quotes(ticker.to_string()).await {
        find_max_min_dates(&stock_data);

        match plot_function(stock_data, &ticker) {
            Ok(_) => println!("Plot generated successfully."),
            Err(e) => eprintln!("Failed to generate plot: {}", e),
        }
    } else {
        eprintln!("Failed to retrieve stock data.");
    }
}

// Function to get the ticker symbol from the command line arguments
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

// Function to retrieve stock quotes for the given ticker symbol
async fn get_stock_quotes(queried_ticker: String) -> Result<Vec<Bar>, Box<dyn std::error::Error>>{
    // Attempt to retrieve stock quotes for the given ticker symbol using the yahoo_finance crate
    let data = match history::retrieve_interval(&queried_ticker, Interval::_6mo).await {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Could not retrieve desired {} stock quotes from Yahoo", queried_ticker);
            return Err(Box::new(err));
        }
    };

    Ok(data)
}


// Function to find and print the maximum and minimum closing prices with their respective dates
fn find_max_min_dates(stock_data: &[Bar]) {
    // Find the maximum and minimum closing prices
    let max_price = stock_data.iter().map(|x| x.high).fold(0./0., f64::max);
    let min_price = stock_data.iter().map(|x| x.low).fold(f64::INFINITY, f64::min);

    // Create a default date as a fallback
    let default_date = Utc.with_ymd_and_hms(2000, 1, 1, 0, 0, 0).unwrap();

    // Find the date associated with the maximum and minimum closing prices
    let max_price_date = stock_data.iter()
        .find(|&bar| bar.high == max_price)
        .map(|bar| bar.datetime())
        .unwrap_or(default_date)
        .to_string();

    let min_price_date = stock_data.iter()
        .find(|&bar| bar.low == min_price)
        .map(|bar| bar.datetime())
        .unwrap_or(default_date)
        .to_string();

    // Print the maximum and minimum closing prices along with their respective dates
    println!("Minimum Closing Price: {:.2}, Date: {}", min_price, min_price_date);
    println!("Maximum Closing Price: {:.2}, Date: {}", max_price, max_price_date);
}


// Function to plot the stock price data
fn plot_function(stock_data: Vec<Bar>, ticker: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize HTML back-end
    let root_area = SVGBackend::new("stock_chart.html", (1280, 720)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    // Determine the minimum and maximum dates from the stock data for the x-axis bounds
    let min_date = stock_data.first().unwrap().datetime();
    let max_date = stock_data.last().unwrap().datetime();

    // Determine the minimum and maximum prices from the stock data for the y-axis bounds
    let max_price = stock_data.iter().map(|x| x.high).fold(0./0., f64::max);
    let min_price = stock_data.iter().map(|x| x.low).fold(f64::INFINITY, f64::min);

    // Generate a caption for the chart including the ticker symbol
    let caption = format!("{} Stock Price Over Last Six Months", ticker);

    // Build a chart with the specified dimensions and label configurations
    let mut chart = ChartBuilder::on(&root_area)
        .caption(caption, ("sans-serif", 50).into_font())
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(min_date..max_date, min_price..max_price)?;

    chart.configure_mesh().draw()?;

    // Draw a line series representing the stock prices over time
    chart.draw_series(LineSeries::new(
        stock_data.iter().map(|bar| (bar.datetime(), bar.close)),
        &RED,
    ))?;

    // Draw point series for bars where the price variation is greater than 2%
    for bar in stock_data.iter().filter(|bar| (bar.high - bar.low) / bar.close > 0.02) {
        // Draw the point at the closing price
        chart.draw_series(PointSeries::of_element(
            [(bar.datetime(), bar.close)], // Coordinates of the point
            5, // Size of the point
            &BLUE, 
            &|coord, size, style| {
                EmptyElement::at(coord) + Circle::new((0, 0), size, style.filled()) // Draw a filled circle at the point
            },
        ))?;

        // Draw the error bars indicating the range between the minimum and maximum prices
        chart.draw_series(
            vec![(
                (bar.datetime(), bar.low), // Starting point of the error bar
                (bar.datetime(), bar.high), // Ending point of the error bar
                &BLUE, 
            )]
            .iter()
            .map(|(start, end, color)| PathElement::new(vec![*start, *end], color)), // Create a path element for the error bar
        )?;
    }

    Ok(())
}
