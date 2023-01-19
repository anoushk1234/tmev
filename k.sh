curl http://0.0.0.0:8899 -X POST -H "Content-Type: application/json" -d '
  {"jsonrpc": "2.0","id":1,"method":"getBlock","params":[5, {"encoding": 
"json","maxSupportedTransactionVersion":0,"transactionDetails":"full","rewards":true}]}
'
