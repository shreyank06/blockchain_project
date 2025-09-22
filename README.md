# Blockchain Project
A bot in Rust that detects potential arbitrage opportunities on the Polygon network. Arbitrage, in this context, means finding a situation where a specific token (e.g., USDC, ETH, BTC, or any other token) can be bought cheaply on one Decentralized Exchange (DEX) and immediately sold for a higher price on another DEX.

## System Architecture
access files this for system architecture diagram and its summary in the project directory.
```
system_architecture.png
system_architecture_summary.txt
```
---
## Execution locally

Clone the repo, and from inside the project directory:

1. **Building and Running the Modules Locally**

    1.1. Create the database file if not already there:
    ```bash
    touch opportunities.db
    ```

    1.2. Build the project:
    ```bash
    cargo build
    ```

    1.3. Run the project:
    ```bash
    cargo run
    ```
    It will do 10 iterations for calculating potential arbitrage opportunities, make an entry for each in the database, and then terminate the program.

2. **Database Entry Check Locally**

    2.1. Open SQLite shell:
    ```bash
    sqlite3 opportunities.db
    ```

    2.2. List the tables:
    ```bash
    .tables
    ```

    2.3. Print all the rows from the `opportunities` table:
    ```bash
    SELECT * FROM opportunities;
    ```
---
## Execution in Docker

3. **Building and Running the Modules in Isolation in Docker Environment**

    3.1. Build the Docker image:
    ```bash
    docker build -t arbitrage-app .
    ```

    3.2. Run the container:
    ```bash
    docker run arbitrage-app sh -c "cargo run && tail -f /dev/null"
    ```

4. **Database Entry Check Inside Docker**

    4.1. Get the container name with in other terminal:
    ```bash
    docker ps
    ```

    4.2. Then access the SQLite shell inside the container:
    ```bash
    docker exec -it <container_id_or_name> sqlite3 opportunities.db
    ```

    4.3. Inside the SQLite terminal, list the tables:
    ```bash
    .tables
    ```

    4.4. Print all the rows from the `opportunities` table:
    ```bash
    SELECT * FROM opportunities;
    ```

---
