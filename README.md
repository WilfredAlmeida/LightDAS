# LightDAS
LightDAS is a lighter version of the [Metaplex Digital Asset RPC API](https://github.com/metaplex-foundation/digital-asset-rpc-infrastructure)

It allows you to index specific Merkle Trees that you care about. This repository works as a listener and ingester for changes on the Merkle Trees you specify. It does the following:
- Listen on the Merkle Tree address via RPC websockets
- Parse a transaction and deserialize its data, events
- Upsert the Metaplex's DAS database
![LightDAS drawio](https://github.com/WilfredAlmeida/LightDAS/assets/60785452/323da5a6-de11-45a0-bdd2-e5b28d547e71)

With LightDAS, you can have your own DAS API without the nft ingester or any other heavy lifting. The components you need to get your DAS running are:
- LightDAS ingester (us)
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
- Install dependencies by `yarn install`
- Add environment variables:
  - `RPC_URL`: RPC needs to support websocket functions. We've built using [Quicknode](https://www.quicknode.com/?via=aayush)
  - `WS_URL`: RPC websocket URL
  - `DATABASE_URL`: Default is `postgres://solana:solana@localhost:5432/solana`, use this unless you changed anything
- Pull db schema into prisma using `npx prisma db pull`
- Generate prisma client using `npx prisma generate`
- Add your Merkle Trees addresses in `src.index.ts`. Existing ones are mostly addresses of scam NFT mints. These trees have high activity and fill up fast
- Start the script `yarn dev`
- It'll print the tree addresses and start the listening to updates on the addresses
- You'll see transaction signatures in the logs
- Under heavy loads, we have faced RPC rate limits
- You can inspect the database via Prisma Studio by running `npx prisma studio`
- RPC Costs per NFT Mint:
  - Quicknode:
    - `logsSubscribe`: 50 credits
    - `getTransaction`: 50 credits
- Overall, each NFT mint will cost you 100 RPC credits

**Currently LightDAS supports only Compressed NFTs with the following instructions**:
- `mintToCollectionV1`

### Testing
If the program is running without any errors then the database is populated with information on new NFT mints. You can query the RPC API locally. It runs on the default URL `http://localhost:9090/`


### Support
If you need any help, have any thoughts, or need to get in touch, DM [Wilfred](https://twitter.com/WilfredAlmeida_) on Twitter/X or open an issue.

The following RFCs are open. We need your thoughts:

[RFC-1: Do we need a Message Queue](https://github.com/WilfredAlmeida/ldas/issues/2)

### Roadmap
The following is our roadmap in decreasing order of priority:  
- Support more instructions
- Rewrite in Rust and move away from TypeScript
- Test out if LightDAS can work as a full fledged DAS. Since we're watching Merkle trees, we can also watch the Bubblegum program and index all NFT operations.

### The Future of LightDAS
Our vision for LightDAS is to keep it an open-source public good for everyone. Currently, we don't have any plans to start a SaaS and compete in the space. We will continue to develop and maintain it as long as we can. The future decisions for LightDAS will be based on community feedback and discussions.

To keep building LightDAS, we need your support and thoughts. It can be contributions, money/grants, hiring us, providing us with resources, etc. Get in touch.

### Humans at LightDAS
[Wilfred Almeida](https://twitter.com/WilfredAlmeida_)  
[Kartik Soneji](https://github.com/KartikSoneji)
