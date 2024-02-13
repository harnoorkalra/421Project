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

2. <financial analysis algorithm>

3. <charting setup>

4. Project setup - The main function starts by calling the function get_ticker_from_command_line() to obtain ticker as user input to be used for stock analysis which uses the clap crate yo parse command line arguments.
Based on the user input, main function then calls the get_stock_quotes with the ticker as a parameter. It uses the yahoo-finance crate to obtain historical data of the given ticker and sort the obtained data into different vectors.
Once the sorted data structures are returned, the main function then calls the plot_function method to plot the required data onto a chart using the plotters crate highlighting the volatile days and printing the min/max stock price days.  
