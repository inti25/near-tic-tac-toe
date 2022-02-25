const { keyStores, connect } = require("near-api-js");
const fs = require("fs");
const path = require("path");
const homedir = require("os").homedir();

const CREDENTIALS_DIR = ".near-credentials";
const ACCOUNT_ID = "inti_demo.testnet";
const WASM_PATH = "contract/target/wasm32-unknown-unknown/release/tic_tac_toe.wasm";

const credentialsPath = path.join(homedir, CREDENTIALS_DIR);
const keyStore = new keyStores.UnencryptedFileSystemKeyStore(credentialsPath);

const config = {
  keyStore,
  networkId: "testnet",
  nodeUrl: "https://rpc.testnet.near.org",
};

async function main() {
  const near = await connect(config);
  const account = await near.account(ACCOUNT_ID);
  const result = await account.deployContract(fs.readFileSync(WASM_PATH));
  console.log(result);
}
main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
