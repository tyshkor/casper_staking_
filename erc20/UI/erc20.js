// var ERC20Client = require('casper-erc20-js-client');
// var CasperContractClient = require('casper-js-client-helper');
// var Keys, CLValueBuilder, CLPublicKey, CLPublicKeyType = require('casper-js-sdk');
const ERC20 = require('casper-erc20-js-client');
const { ERC20Client } = ERC20;
// const KEYS = Keys.Ed25519.parseKeyFiles(
//   './keys/public_key.pem',
//   './keys/secret_key.pem',
// );
const KEYS = 'MC4CAQAwBQYDK2VwBCIEIJEoNtQENWy3J4Td/BAa912jdHI1Gf9NAuy8k3NNNUmq\r\nMCowBQYDK2VwAyEAf7vM85pjmhpfRp4/shDZ81W1Mr14b5RUCfD8moxjE7E=\r\n';
const RPC = "http://159.65.118.250:7777";
const network = "casper-net-1";
const stream = "http://159.65.118.250:7777/events/main";

async function initializeContract(){
  const erc20 = new ERC20Client(
    RPC, // RPC address
    network, // Network name
    stream // Event stream address
  );

  console.log("ERC20 Instance initialized", erc20);

  return erc20;
}

// async function deployContract(erc20){
//   const installDeployHash = await erc20.install(
//     KEYS, // Key pair used for signing 
//     "FerrumX", // Name of the token
//     "FRMX", // Token Symbol
//     11, // Token decimals
//     1000000000000000, // Token supply
//     200000000000, // Payment amount
//     "../erc20/target/wasm32-unknown-unknown/release/erc20_token.wasm" // Path to WASM file
//   );
//   return installDeployHash;
// }

// Call start
(async() => {
  const erc20 = await initializeContract();
    const installDeployHash = await erc20.install(
    KEYS, // Key pair used for signing 
    "FerrumX", // Name of the token
    "FRMX", // Token Symbol
    11, // Token decimals
    1000000000000000, // Token supply
    200000000000, // Payment amount
    "../erc20/target/wasm32-unknown-unknown/release/erc20_token.wasm" // Path to WASM file
  );

  // console.log(erc20);

  // const installDeployHash = deployContract(erc20);

  // console.log(installDeployHash);
})();



// const erc20 = initializeContract();
// console.log(erc20);

// const name = await erc20.name();
// console.log(name);

  // await erc20.setContractHash('hash-c2402c3d88b13f14390ff46fde9c06b8590c9e45a9802f7fb8a2674ff9c1e5b1');
  
  // const name = await erc20.name();

  // const symbol = await erc20.symbol();
  
  // const totalSupply = await erc20.totalSupply();
  
  // const decimals = await erc20.decimals();

  // casper-client put-deploy \
  // --chain-name casper-test \ 
  // --node-address http://159.65.118.250:7777 \
  // --secret-key ./keys/secret_key.pem \
  // --session-path target/wasm32-unknown-unknown/release/erc20_token.wasm \
  // --payment-amount 80000000000 \
  // --session-arg "name:string=‘FerrumX'" \
  // --session-arg "symbol:string=‘FRMX'" \
  // --session-arg "total_supply:u256='100'" \
  // --session-arg "decimals:u8='8'"