# Crowdfunding DCDT - Using DCDT (Fungible token)

First [set up a node terminal](../../../../tutorial/src/interaction/interaction-basic.md).

```javascript
let moajs = await require('@numbatnetwork/moajs');
let { moaSys, wallets: { alice, bob, carol } } = await moajs.setupInteractive("local-testnet");

let crowdfunding = await moaSys.loadWrapper("contracts/examples/crowdfunding-dcdt");

// Issue a new fungible token
let MyToken = await moaSys.sender(alice).issueFungible("MyFungibleToken", "MYTOKEN", 1_000_00, 2);

// Note: wait a few seconds, otherwise the first send fails
// Send some tokens to bob and carol (so that they can call fund later on)
await moaSys.sender(alice).value(MyToken(200.0)).send(bob);
await moaSys.sender(alice).value(MyToken(200.0)).send(carol);

// Set the deadline to 1 minute from now (adjust this if you want more time before claiming the rewards)
let someTimeFromNow = await moaSys.currentNonce() + moajs.minutesToNonce(1);

// Deploy the crowdfunding contract
await crowdfunding.sender(alice).gas(50_000_000).call.deploy(MyToken(2), someTimeFromNow, MyToken);

// Bob and carol contribute 1.5 MYTOKEN each
await crowdfunding.sender(bob).gas(10_000_000).value(MyToken(1.5)).call.fund();
await crowdfunding.sender(carol).value(MyToken(1.5)).call.fund();

// Get the current funds. Note the usage of MyToken.raw (since the balance comes as an integer from the smart contract)
let currentFunds = MyToken.raw(await crowdfunding.query.currentFunds());

// Should print 3 MYTOKEN (since bob and carol added 1.5 MYTOKEN each)
moajs.print(currentFunds);

// Confirming the target is 2 MYTOKEN
moajs.print(MyToken.raw(await crowdfunding.query.get_target()));

// Check that alice is the owner
alice.address.equals(await crowdfunding.query.get_owner());

// Store alice's current balance (we'll use this to check the balance difference later on)
let aliceBalanceBefore = await moaSys.getBalance(alice, MyToken);
moajs.print(aliceBalanceBefore);

// Wait a minute first, otherwise you'll get the "cannot claim before deadline" error
// If the claim doesn't return an error - there are two possibilities:
// - the funding failed, and 1.5 MYTOKEN are sent back to both bob and carol
// - it was succesful and alice receives 3 MYTOKEN
// Because the target sum specified on deployment was 2 MYTOKEN, and we have 3 MYTOKEN, the funding should be succesful
await crowdfunding.sender(alice).call.claim();

// Let's check if alice received the funds
let aliceBalanceAfter = await moaSys.getBalance(alice, MyToken);
moajs.print(aliceBalanceAfter);

// If the previous claim was successful, this prints 3 MYTOKEN
moajs.print(aliceBalanceAfter.minus(aliceBalanceBefore));
```
