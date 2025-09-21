# Blockchain Project
A bot in Rust that detects potential arbitrage opportunities on the Polygon network. Arbitrage, in this context, means finding a situation where a specific token (e.g., USDC, ETH, BTC or any other token) can be bought cheaply on one Decentralized Exchange (DEX) and immediately sold for a higher price on another DEX.

## Execution
Clone the repo, and from inside the project directory

1. Building and Running the modules locally

1.1 crate db file if not already there
```
touch opportunities.db
```

1.2 Build
```
cargo build
```


1.3 run
```
cargo run
```
It will do 10 iterations for calculating potential arbirtrage opportunities and make and entry for each in the database and then terminate the program.

2. Database Entry Check Locally

2.1 Open SQLite shell
```
sqlite3 opportunities.db
```
2.2 list the tables
```
.tables
```
2.3 print all the rows from opportunities.db sql table
```
SELECT * FROM opportunities;
```

3.  Building and Running the modules in isolation in docker enviornment
3.1 build
```
docker build -t arbitrage-app .
```
3.2 run
```
docker run arbitrage-app sh -c "cargo run && tail -f /dev/null"
```


4. Database Entry Check inside docker

4.1 get the container name with
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
