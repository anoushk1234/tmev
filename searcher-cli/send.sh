../target/release/jito-searcher-cli \
  --block-engine-url https://amsterdam.mainnet.block-engine.jito.wtf \
  --keypair-path ../id.json \
  send-bundle \
  --payer ../id.json \
  --message "im testing jito bundles right now this is pretty sick bro" \
  --num-txs 1 \
  --lamports 100 \
  --tip-account 96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5 \
  --rpc-url "https://amsterdam.mainnet.rpc.jito.wtf/?access-token=76819d17-b796-4253-b7f5-6c543fb3c508"