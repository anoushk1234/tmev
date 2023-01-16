
git submodule update --init --recursive

cargo b --release && \
    RUST_LOG=info ./target/release/jito-backrun-example \
    --auth-addr https://amsterdam.mainnet.block-engine.jito.wtf \
    --searcher-addr https://amsterdam.mainnet.block-engine.jito.wtf \
    --payer-keypair ./id.json  \
    --auth-keypair ./id.json  \
    --pubsub-url wss://amsterdam.mainnet.rpc.jito.wtf/access-token/76819d17-b796-4253-b7f5-6c543fb3c508 \
    --rpc-url https://amsterdam.mainnet.rpc.jito.wtf/access-token/76819d17-b796-4253-b7f5-6c543fb3c508 \
    --tip-program-id 7JCWzUcPQvA4PHAWzeckkDgfCMZHu9c42LzULg6cC2N8 \
    --backrun-accounts 6hPA69XMwMnUS1yurC2P9z9Fys4x8HMNpZDvaLbCZH9w

    