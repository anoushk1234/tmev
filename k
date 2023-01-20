curl http://api.mainnet-beta.solana.com -X POST -H "Content-Type: application/json" -d '
  {"jsonrpc": "2.0","id":1,"method":"getBlock","params":[174572266, {"encoding": "json","maxSupportedTransactionVersion":0,"transactionDetails":"full","rewards":false}]}
'