# CosmWasm Starter Pack
## zero-to-hero-sparkibc

### Polling APP like starpoll on chain 


**Callum's Official Repo**
https://github.com/Callum-A/spark-ibc-cosmwasm-zero-to-hero

**layout**
```
src/ folder (template generated for you)

lib.rs - defines files in the project & exports them for use
contract.rs - where actual contract logic happens (uses messages)
error.rs - contract errors for when logic does not line up with what you want

helpers.rs - we deleted this as it is not needed, also removed from lib.rs

msg.rs - holds messages which we use in the project & has more info in there.

state.rs - storage of the contract via Maps / Key Value Stores
```

**notes**
notes can be found in each contract rust file (*.rs)