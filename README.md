---
## 🍕🍕 Pizza Tipper!
A simple MVP for a social platform for tipping content creators !
Every tip is about the cost of a pizza.
Content creators will post content and have the ability to highlight their content.
Tippers will tip creators and have the ability to highlight their tip.

## Pizza cost
Pizza cost is determined by the pizza oracle. This is a smart contract. Currently in development.


## Azero ID 
Users (both creators and tippers) will have the option to post content linked to their
azero id or to tip  another user's azero id, respectively

## References
Used the following to help me build smart contracts in ink:

https://github.com/scio-labs/inkathon

https://github.com/Cardinal-Cryptography/bulletin-board-example

https://github.com/paritytech/ink-examples/tree/main

https://github.com/azero-id/contract-integration/tree/main

https://use.ink/

## Tests
![Screenshot from 2023-10-27 15-36-15](https://github.com/nuts-rice/pizza_tipper/assets/5147364/7d2d1267-9253-448e-8f89-ca6ac0f54c13)

Tests for emitting test on tip, constructor, checks for tip has the right value transferred
Also has e2e tests for getting list of tippers from client
