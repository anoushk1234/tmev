The folder contains the REST API and also the bundle stream server

# Bundle Stream

 - Captures all bundles from latest blocks that have been submitted by all searchers.
 - Uses gRPC streams to serve the latets bundles that have landed
 - Built on Jito's Starrider RPC
 - Uses existing RPC Infra and can easily be tweaked for Jito's upcoming gRPC block stream based on geyser.

# Usage
- run `sh watch.sh` if you have cargo watch or `cargo r -r`
