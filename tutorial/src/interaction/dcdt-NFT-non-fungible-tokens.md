# Non-Fungible Tokens (NFTs)

Non-Fungible Tokens have amounts of either 0 or 1, and variable nonce. They are not denominated (the amount has 0 decimals).

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let moajs = await require('@numbatnetwork/moajs');
let { moaSys, Rewa, wallets: { alice, bob, carol } } = await moajs.setupInteractive("local-testnet");

// Issue a new non-fungible token
let MyToken = await moaSys.sender(alice).issueNonFungible("MyFungibleToken", "MYTOKEN");

// Check the token's identifier
console.log(MyToken.getTokenIdentifier());

await moaSys.dcdtSystemContract.sender(alice).call.setSpecialRole(MyToken, alice, "DCDTRoleNFTCreate");

// Create 2 tokens
let MyFirstNFT = await moaSys.sender(alice).dcdtNftCreate(MyToken, 1, "MyFirstNFT", 0, "", "", "https://example.com");
let MySecondNFT = await moaSys.sender(alice).dcdtNftCreate(MyToken, 1, "MySecondNFT", 0, "", "", "https://example.com");

// Send some tokens to bob and carol
await moaSys.sender(alice).value(MyFirstNFT.one()).send(bob);
await moaSys.sender(alice).value(MySecondNFT.one()).send(carol);

await moaSys.getBalanceList(alice, MyToken).then(moajs.printList);
await moaSys.getBalanceList(bob, MyToken).then(moajs.printList);
```
