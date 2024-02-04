import { PublicKey } from "@solana/web3.js";

import { connection } from "./rpc/rpc";
import {
  processTransaction,
  processWebsocketMerkleTreeTransaction,
} from "./processors/processor";

const main = async () => {
  const trees = [
    { address: "4iEJbvSdEJK84uFgD47J47eXSZuRFKvUFBRVicHgJYBc" },
    { address: "GaRRBSZMRUFZaBaNi66PRQNgFzMcNJpDKrMzuwhdHdRR" },
    { address: "5cCPghGZBmMRoHCu6MvFVWs8z41CbyibmKDxzY4gSLdk" },
    { address: "6irWsFQajvBgpbx99bzuu2HX6mqA4AocHWpFt5AhW3TF" },
    { address: "9kQaT7Gg3EsFm8P51Gz9vBu1dyPTZxWWCGd6rACDgED" },
    { address: "AYFUCqQXcT5zupPhXB6hFVrz89cRQxB5gD1FvYedeUpy" },
    { address: "Az8yR39sHMZwjsQBvKbA5YyWFcd9EEkaDtiDt8Qu5P5x" },
    { address: "B1eWW3tTBb5DHrwVrqJximAYLwucGzvjuJWxkFAe4v2X" },
    { address: "G4xR1XnzntLkZvHbY9ibnp9uDahYvBsBCn3bYhiNF44H" },
    { address: "YE7wy11nJobja8QZrQz2dYBdAdFNLBFyooC5NHRUVd3" },
    { address: "9LBMrf2xbRoSBJruDhfAm4QSkafqySmiSh4tweEENrtW" },
    { address: "2WQhqhYz8MZGGEiraPcYGuDemnGGJGsCkS54kD8yjnSa" },
    { address: "C7Hp7t9ojf4r9hFcY1zT1VXg7GjgER6oUdHUCF9SyjZ1" },
    { address: "3gz5ZD8ZsZtAHMhxAYoTXfW5yReio53ALCwpT2CC4tYv" },
    { address: "5XcnDce9Do8HKNAuWCPKJpGnhazhs3anbK6PQwt1fDLg" },
    { address: "Cu61XHSkbasbvBc3atv5NUMz6C8FYmocNkH7mtjLFjR7" },
    { address: "C8vRoSDJRUG2mjhKYA5iHZk1odL5vRc89nZu7LXKx3yr" },
    { address: "GHh4bn52FvfS2eHQ4LaQyeTAETDvd4Y2RvgjQwD6ZHVj" },
    { address: "E4Jdig6NR54rDTC7XP72aGrexmMe6bADc4RYqH3eJjYZ" },
    { address: "Hm1JSra1i2Wfk71YnnnrRxtCpkTAdniAPaVsrvvdb6wE" },
    { address: "E4Jdig6NR54rDTC7XP72aGrexmMe6bADc4RYqH3eJjYZ" },
    { address: "tS2srkdCZeqnzrFt25PbBFg4KoVJB4WtPUJnVidsY4p" },
    { address: "E4Jdig6NR54rDTC7XP72aGrexmMe6bADc4RYqH3eJjYZ" },
    { address: "trENTgAwckNbFLKxgLtwzcNBSjFpPcsC23LsqGNvbgN" },
    { address: "Cf9Cqg3bvWiQV1euponc23F5su6pTD2yXA2zT5nqGfdw" },
    { address: "Aju7YfPdhjaqJbRdow48PqxcWutDDHWww6eoDC9PVY7m" },
    { address: "5vaKxbmSWaS716M4XLxY2cPqaLBxEn7wkiVZsZGs2t8z" },

    // devnet
    // { address: "r6frHpei4tXkNytXEjbBgtzZVEPtGKg338Q3rrHFEnk" },
    // { address: "GsxSsXQ6pxN5mBbTWYVkwUcDcHWPgVTRsd3TMo63ArsX" },
    // { address: "GqwJMqkoyRvprwRPSRFjPPkQX5svRqaasAmhRPNumFj2" },
    // { address: "2LVSTQPMQouSUa2KfkY7Mimzwi7QiGeshEXmhy9NGNNn" },
    // { address: "4cZQDeDYcVrhiRdVKg8qMPNEX9WyLiL3z3fiAV9rsi5y" },
  ];

  for (let i = 0; i < trees.length; i++) {
    const tree = trees[i];
    const merkleTreeAddress = new PublicKey(tree.address);

    // `onLogs` is triggered by a websocket when anything happens on the given Merkle Tree address.
    // The handler functions takes the transaction signature and processes it.
    console.log(merkleTreeAddress.toBase58());

    connection.onLogs(
      merkleTreeAddress,
      async (logs) => {

        const { lastProcessedTxSignature } =
          await processWebsocketMerkleTreeTransaction(logs.signature);

        console.log("lastProcessedTxSignature: ", lastProcessedTxSignature);
      },
      "confirmed"
    );
  }
};

main();
