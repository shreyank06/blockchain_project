# Blockchain Project
A bot in Rust that detects potential arbitrage opportunities on the Polygon network. Arbitrage, in this context, means finding a situation where a specific token (e.g., USDC, ETH, BTC or any other token) can be bought cheaply on one Decentralized Exchange (DEX) and immediately sold for a higher price on another DEX.

### Building and Running the modules

crate db file
```
touch opportunities.db
```

Build
```
cargo build
```


run
```
cargo run
```
It will do 10 iterations for calculating potential arbirtrage opportunities and make and entry for each in the database.

### Database Entry Check
Open SQLite shell
```
sqlite3 opportunities.db
```
list the tables
```
.tables
```
print all the rows from opportunities.db sql table
```
SELECT * FROM opportunities;
```
