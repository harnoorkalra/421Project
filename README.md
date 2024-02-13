# ECE 421 Project - Stock Market Monitor

Collaborators:
Noor Kalra
Kendrick Schellenberg
Nnamdi Ajoku


clap = "4.5.0"
plotters = "0.3.5"
yahoo-finance = "0.3.0"
tokio = "0.2.1"

1. Crates used:
   (i). clap - It is used to parse command line arguments and options
   (ii). plotters - It is a data visualization library to plot charts and plots.
   (iii). yahoo-finance - It is used to fetch financial market data (stock quotes in this project) from the Yahoo Finance public api.
   (iv). tokio - It is an asynchronous runtime for Rust that enables the execution of concurrent operations without threading (fetching data from Yahoo Finance in this project).

2. Financial analysis algorithm
The financial analysis algorithm we used for the default 'volatile' days analysis. The algorithm identifies when a day's stock price varied by more than 2% of the total price (as measured by the difference between the intra-day high and low). The purpose of this identification is to help an analyst predict when periods of high volatility will occur, as those will hold the possibility for greater losses and gains.

3. <charting setup>

4. Project setup - The main function starts by calling the function get_ticker_from_command_line() to obtain ticker as user input to be used for stock analysis which uses the clap crate yo parse command line arguments.
Based on the user input, main function then calls the get_stock_quotes with the ticker as a parameter. It uses the yahoo-finance crate to obtain historical data of the given ticker and sort the obtained data into different vectors.
Once the sorted data structures are returned, the main function then calls the plot_function method to plot the required data onto a chart using the plotters crate highlighting the volatile days and printing the min/max stock price days.  

5. Usage instructions
Assuming the stock_monitor folder has been downloaded, the package must be built using Cargo. Navigate to the folder in the terminal, and enter 'cargo build'. This will download the dependencies, as listed in Cargo.toml, and prepare the package to be run. Upon completion of the build, the program can be run in two ways
- Navigate to the target/debug folder and run stock_monitor.exe with the desired ticker. For example './stock_monitor --ticker AAPL'
- While in the stock_monitor folder in the terminal, use cargo run with the desired ticker. For example 'cargo run -- --ticker AAPL'
