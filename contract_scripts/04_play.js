const { connect, KeyPair, keyStores, utils } = require("near-api-js");
const fs = require("fs");
const path = require("path");
const homedir = require("os").homedir();

const CREDENTIALS_DIR = ".near-credentials";
const CONTRACT_ID = "inti_demo.testnet";
const PLAYER1 = "inti.testnet";
const PLAYER2 = "inti01.testnet";
const credentialsPath = path.join(homedir, CREDENTIALS_DIR);
const keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);

const config = {
  keyStore,
  networkId: "testnet",
  nodeUrl: "https://rpc.testnet.near.org",
};

async function main() {
  const near = await connect(config);
  const account = await near.account(PLAYER1);
  const result = await account.functionCall({
    contractId: CONTRACT_ID,
    methodName: "play",
    args: {'game_id': '1', 'position': 1}
  })
  console.log(result);
}
main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
