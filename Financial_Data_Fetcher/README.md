This is a RUST program that fetches the current price of bitcoing, ethereum, and the S&P 500
index every 10 seconds and saves each price into a separate text files


API's that I used:
  - CoinGecko for bitcoing and ethereum
  - Yahoo Finance (for SP500)

Libraries I used:
- ureq
- serde


This program runs by typing:
- cargo run
and it should output if each asset was saved or fetched successfully or not

To check if ran successfully:
- check the following files: bitcoin.txt, ethereum.txt, and sp500.txt
