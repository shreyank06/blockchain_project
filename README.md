# Blockchain Project
A bot in Rust that detects potential arbitrage opportunities on the Polygon network. Arbitrage, in this context, means finding a situation where a specific token (e.g., USDC, ETH, BTC or any other token) can be bought cheaply on one Decentralized Exchange (DEX) and immediately sold for a higher price on another DEX.

## Execution
Clone the repo, and from inside the project directory

### Building and Running the modules locally

crate db file if not already there
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
It will do 10 iterations for calculating potential arbirtrage opportunities and make and entry for each in the database and then terminate the program.

#### Database Entry Check Locally

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

### Building and Running the modules in isolation in docker enviornment
build
```
docker build -t arbitrage-app .
```
run
```
docker run arbitrage-app sh -c "cargo run && tail -f /dev/null"
```


#### Database Entry Check Locally

get the container name with
```
docker ps
```
then
```
docker exec -it <container_id_or_name> sqlite3 opportunities.db
```
then inside sqlie termnal
```
.tables
```
```
SELECT * FROM opportunities;
```
