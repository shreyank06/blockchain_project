# Blockchain Project
A bot in Rust that detects potential arbitrage opportunities on the Polygon network. Arbitrage, in this context, means finding a situation where a specific token (e.g., USDC, ETH, BTC or any other token) can be bought cheaply on one Decentralized Exchange (DEX) and immediately sold for a higher price on another DEX.

### Building and Running the modules locally

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
It will do 10 iterations for calculating potential arbirtrage opportunities and make and entry for each in the database and then terminate the program.

### Building and Running the modules in isolation
build
```
docker build -t arbitrage-app .
```
run
```
docker run arbitrage-app cargo run1
```


### Database Entry Check
#### Locally

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

#### Inside Docker
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
