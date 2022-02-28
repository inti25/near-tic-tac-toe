# NEAR Tic Tac Toe

## Description
This contract implements simple online game use near sdk
Contract in `contract/src/lib.rs` 

## Demo
https://inti25.github.io/near-tic-tac-toe/index.html

## Setup 
Install dependencies:

```
npm install
```

If you don't have `Rust` installed, complete the following 3 steps:

1) Install Rustup by running:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

([Taken from official installation guide](https://www.rust-lang.org/tools/install))

2) Configure your current shell by running:

```
source $HOME/.cargo/env
```

3) Add wasm target to your toolchain by running:

```
rustup target add wasm32-unknown-unknown
```

Next, make sure you have `near-cli` by running:

```
near --version
```

If you need to install `near-cli`:

```
npm install near-cli -g
```

## Login
If you do not have a NEAR account, please create one with [NEAR Wallet](https://wallet.testnet.near.org).

In the project root, login with `near-cli` by following the instructions after this command:

```
near login
```

Modify the top of `contract_scripts/*.js`, changing the `CONTRACT_NAME and ACCOUNT_ID` to be the NEAR account that was just used to log in.

```javascript
const ACCOUNT_ID = 'YOUR_ACCOUNT_NAME_HERE'; /* TODO: fill this in! */
const CONTRACT_ID = 'YOUR_ACCOUNT_NAME_HERE'; /* TODO: fill this in! */
```

## To Build the SmartContract

```shell
cd contract
./complie.sh
```

## To Deploy the SmartContract

```shell
node contract_scripts/01_deploy.js
```
## To run front-end
```shell
npm run start
```
## To Explore

- `contract/src/lib.rs` for the contract code include init function and change method
- `contract/src/views.rs` for the contract code include view method
- `src/index.html` for the front-end HTML
- `src/main.js` for the JavaScript front-end code and how to integrate contracts