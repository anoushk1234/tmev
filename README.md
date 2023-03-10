# Tmev
A searcher's terminal UI dashboard built using [tui.rs](https://docs.rs/tui/latest/tui/) crate


## Prelude
Jito solana is a validator optimised for mev on solana. The way it works is jito has a block engine that gets transactions forwarded by the relayer, once it gets the pending txns the searchers can group these into optimised bundles and include a tip for doing so, the bundles are then sent to the validator and the transactions are executed

## So what are we solving
We're solving two problems:
- searchers need a quick terminal ui to explore arbitrage txns, liquidations, and the bundles that are being sent to the validator so they can see their bundles and also inspect bundles sent by other searchers, for this we built [tmev-cli](https://github.com/anoushk1234/tmev/tmev-cli), demo: https://www.loom.com/share/4abb634327ef4575852b0975ac09e890
- there's no way to read sent bundles from the block engine other than being a validator so we built our own gRPC server with a custom bundle parsing algorithm that parses the latest blocks from Jito's RPC and runs it through our algo to parse them into bundles and stream it to your client, check it out in [bundle-stream](https://github.com/anoushk1234/tmev/blob/master/searcher-api/src/main.rs) demo: https://www.loom.com/share/a53e256124ee48baa6e0bc4f8f8e6d8c

## Tmev cli app 
 + Queries the latest arbitrages from Jito's MEV dashboard and displays it in a nice tabular form. In addition to that, 
 + Displays the bundles sent by all the searchers in the network .
 + Displays current searchers' sent bundles queried by their public key.
 + Displays tips earned by individual bundles with a pretty ui.
 + Built in rust for blazingly(lol) fast cli that doesn't take all your ram.


#### Usage
- Add a .env in `searcher-api`, refer .env.example .
- Run `cargo r -r` or `sh watch.sh` in `searcher-api/` as this will start the server.
- Run ```cargo r -r -- --arbs <searcher-pubkey>``` or ```sh install.sh``` in your terminal 
- After the terminal displays you could use ```Left``` or ```Right``` arrow keys to navigate between tabs.
- To scroll use ```Up``` or ```Down``` arrow keys and to quit press ```Q```.

## Bundle Stream
 - Captures all bundles from latest blocks that have been submitted by all searchers.
 - Uses gRPC streams to serve the latets bundles that have landed
 - Built on Jito's Starrider RPC
 - Uses existing RPC Infra and can easily be tweaked for Jito's upcoming gRPC block stream based on geyser.
