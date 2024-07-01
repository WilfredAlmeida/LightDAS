# LightDAS
LightDAS is a lighter version of the [Metaplex Digital Asset RPC API](https://github.com/metaplex-foundation/digital-asset-rpc-infrastructure)

**[MUST WATCH DEMO](https://www.loom.com/share/cdea6acd488d4202a16992b45b6e25d1?sid=112a4cc3-4f67-4e1f-a60d-f84c5fad59e2)**  
**[Pitch Deck](https://pitch.com/v/lightdas-gjrunw)**

It allows you to index specific Merkle Trees that you care about. This repository works as a listener and ingester for changes on the Merkle Trees you specify. It does the following:
- Listen on the Merkle Tree address via RPC websockets
- Parse a transaction and deserialize its data, events
- Upsert the Metaplex's DAS database
![LightDAS drawio](https://github.com/WilfredAlmeida/LightDAS/assets/60785452/323da5a6-de11-45a0-bdd2-e5b28d547e71)

### Reasons we are building LigthDAS
- Running a standard DAS API is expensive and complicated
- It gives you data off all of the NFTs on chain, but do you really need all of it?
- There are select DAS offerings thus creating a monopolistic environment

With LightDAS, you can have your own DAS API without the nft ingester or any other heavy lifting. The components you need to get your DAS running are:
- LightDAS ingester (us)
- DAS Backfiller (Metaplex)
- DAS API Handler (Metaplex)
- DAS Database (Metaplex)
- Graphite Monitoring (Metaplex)

## Getting started
Follow the steps mentioned below

### Metaplex DAS
- Clone the [Metaplex Digital Asset RPC API](https://github.com/metaplex-foundation/digital-asset-rpc-infrastructure) repo
- You need the `api`, `db`, and `graphite` containers
- Run `docker compose up`. This'll take some time to build and start the containers. Depending on your machine, you can comment out services in the `docker-compose.yaml` if you want them built
- After the build is successful, you can stop all other containers except the ones mentioned above
- Then configure and run LightDAS

### LightDAS
- Clone the repo
- Add environment variables:
  - `RPC_URL`: RPC needs to support websocket functions. We've built using [Quicknode](https://www.quicknode.com/?via=aayush)
  - `WS_URL`: RPC websocket URL
  - `DATABASE_URL`: Default is `postgres://solana:solana@localhost:5432/solana`, use this unless you changed anything
- Execute `cargo run`
- This will download and compile the code with all needed dependencies. Grab a coffee this takes a while
- The first run will fail and complain about no tree addresses being found to index, you need to add tree addresses to index in the database. See the `#trees config` section below
- Once running, you'll see the logs of the tasks being performed
- Under heavy loads, we have faced RPC rate limits
- RPC Costs per NFT Mint:
  - Quicknode:
    - `logsSubscribe`: 50 credits
    - `getTransaction`: 50 credits
- Overall, each NFT mint will cost you 100 RPC credits

### Trees Config
1. The address of the trees to be indexed needs to be provided via the database
2. LightDAS creates a table with the following schema 
   ```
   CREATE TABLE IF NOT EXISTS LD_MERKLE_TREES (
      ADDRESS VARCHAR(255),
      TAG VARCHAR(255) NULL,
      CAPACITY INT NULL,
      MAX_DEPTH INT NULL,
      CANOPY_DEPTH INT NULL,
      MAX_BUFFER_SIZE INT NULL,
      SHOULD_INDEX BOOLEAN DEFAULT TRUE,
      CREATED_AT TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
      UPDATED_AT TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );
   ```
3. You need to add your addresses in the table `ld_merkle_trees`
4. To update tree addresses dynamically:
  1. Update the above table
  2. Send a SIGHUP signal to the LightDAS process
  3. LightDAS handles the signal and update it's indexing without disrupting existing tasks

**Currently LightDAS supports only Compressed NFTs**:

### Testing
If the program is running without any errors then the database is populated with information on new NFT mints. You can query the RPC API locally. It runs on the default URL `http://localhost:9090/`


### Support
If you need any help, have any thoughts, or need to get in touch, DM [Wilfred](https://twitter.com/WilfredAlmeida_) on Twitter/X or open an issue.

We have some open [RFCs](https://github.com/WilfredAlmeida/LightDAS/labels/rfc) and need your thoughts  

### Roadmap
The following is our roadmap in decreasing order of priority:  
- Test API responses correctness against standard DAS API responses
- Publish benchmarking results of testing with different RPC providers under various deployment environments

### The Future of LightDAS
Our vision for LightDAS is to keep it an open-source public good for everyone. We aim to be a compliment to DAS, not to compete against it. Eventually, we would like to streamline the setup process to only need a single binary with minimal dependencies so it's easy for a project to setup a light DAS client to watch a tree and start serving requests. The future decisions for LightDAS will be based on community feedback and discussions.

To keep building LightDAS, we need your support and thoughts. It can be contributions, money/grants, hiring us, providing us with resources, etc. Get in touch.

### Licensing
All code is licensed under the GNU Affero General Public License v3.0 or later.

### Humans at LightDAS
[Wilfred Almeida](https://twitter.com/WilfredAlmeida_)  
[Kartik Soneji](https://github.com/KartikSoneji)
