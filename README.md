# avalanche-workshop


# Demo 1: 
- Using `curl` to connect to P-Chain
- Connect to avalanche

## Detail
- 
```bash
curl -X POST --data '{
    "jsonrpc":"2.0",
    "id"     :1,
    "method" :"info.getNodeID"
}' -H 'content-type:application/json' 127.0.0.1:9650/ext/info
```