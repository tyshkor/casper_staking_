# Ferrum Staking - Casper Smart Contracts

This repository contains the smart contracts used for the Staking on the Casper Network.

This contract has the following functionality:

- stake the specific token
- add reward
- see total current reward
- withdraw amount with reward
- see current reward


## Table of Contents

0. [Prerequisites](#prerequisites)

1. [Getting Started](#getting-started)

2. [Usage](#usage)

3. [Installing and Interacting with the Contract using the Rust Casper Client](#installing-and-interacting-with-the-contract-using-the-rust-casper-client)

4. [Events](#events)

5. [Error Codes](#error-codes)

6. [Contributing](#contributing)

## Prerequisites

You need to have a x86_64 CPU to build the code, as Casper dependencies for now don't support ARM (e.g. M1) architecture CPU's.
It is recommended to use Linux, Debian-based distributions (e.g. Ubuntu 22.04).

First, you need to install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

To check for successful installation, one needs to run this command:

```bash
rustup --version.
```

Secondly, you'll need to install `wabt` tooling:

```bash
sudo apt install wabt
```

Thirdly, you need to install CMake:

```bash
sudo apt-get -y install cmake
```

To check for successful installation, one needs to run this command:

```bash
cmake --version
```

To install `casper-client`, to interact with the contract using the CLI you must install these dependencies:

```bash
sudo apt-get install pkg-config
sudo apt-get install openssl
sudo apt-get install libssl-dev
```

And finally:

```bash
cargo install casper-client
```

To check for successful installation, one needs to run this command:

```bash
casper-client get-state-root-hash --node-address http://65.21.235.219:7777
```


## Getting Started

To get started with using the smart contracts in this repository, you will need to have a working environment for Rust and Casper CLI.

```bash
cargo install casper-client
```

## Usage

### Set up the Rust toolchain

```bash
make prepare
```

### Compile smart contracts

```bash
make build-contract
```

### Run tests

```bash
make test
```

### Installing and Interacting with the Contract using the Rust Casper Client

#### Prerequisites

Please, ensure, that your private key is called `secret_key.pem` and is situated in `staking_contract/keys` subdirectory, otherwise you won't be able to run the commands.


##### Example deploy

The following is an example of deploying the installation of the contract via the Rust Casper command client.

```bash
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://44.208.234.65:7777 \
    --secret-key ./staking_contract/keys/secret_key.pem \
    --session-path ./staking_contract/target/wasm32-unknown-unknown/release/staking_contract.wasm \
    --payment-amount 200000000000 \
    --session-arg "name:string='FerrumX'" \
    --session-arg "address:string='782fe4b0bb944e6b1fd2c5a1456a78f0e2193d47dee9b1af5711d6b6e6aaca60'" \
    --session-arg "staking_starts:u64='<milliseconds timestamp>'" \
    --session-arg "staking_ends:u64='<milliseconds timestamp>'" \
    --session-arg "withdraw_starts:u64='<milliseconds timestamp>'" \
    --session-arg "withdraw_ends:u64='<milliseconds timestamp>'" \
    --session-arg "staking_total:U256='<amount of tokens you want to be the staking limit>'" \
    --session-arg "erc20_contract_package_hash:Key='hash-<contract-package-hash for the CEP18 token you want to be staked by this contract>'"
```

##### Example Stake
```bash
casper-client put-deploy \
     --chain-name casper-test \
     --node-address http://44.208.234.65:7777 \
     --secret-key ./staking_contract/keys/secret_key.pem \
     --session-hash hash-<contract-package-hash-of-deployed-coontract> \
     --session-entry-point stake \
     --payment-amount 5000000000 \
     --session-arg "amount:u256='5'" 
```

##### Example get_current_reward
```bash
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://44.208.234.65:7777 \
    --secret-key ./staking_contract/keys/secret_key.pem \
    --session-hash hash-<contract-package-hash-of-deployed-coontract> \
    --session-entry-point get_current_reward \
    --payment-amount 50000000000 
```

##### Example staker_reward
```bash
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://44.208.234.65:7777 \
    --secret-key ./staking_contract/keys/secret_key.pem \
    --session-hash hash-<contract-package-hash-of-deployed-coontract> \
    --session-entry-point staker_reward \
    --payment-amount 50000000000 \
    --session-arg "staker_address:key='hash-8c07f894322d86705f9804d682a9ed6c9cd4be7a8fc6889d20b446e1d852fa8c'"
```

##### Example add_reward
```bash
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://44.208.234.65:7777 \
    --secret-key ./staking_contract/keys/secret_key.pem \
    --session-hash hash-<contract-package-hash-of-deployed-coontract> \
    --session-entry-point add_reward \
    --payment-amount 50000000000 \
    --session-arg "reward_amount:u256='1'" \
    --session-arg "withdrawable_amount:u256='1'"
```

##### Example amount_staked
```bash
casper-client put-deploy \
    --chain-name casper-test \
    --node-address http://44.208.234.65:7777 \
    --secret-key ./staking_contract/keys/secret_key.pem \
    --session-hash hash-0f401c728a28f5df5404640e163e42edf420abd93e6b02a45e4f52524b652a71 \
    --session-entry-point amount_staked \
    --payment-amount 50000000000 \
    --session-arg "amount:u256='5'" \
    --session-arg "staker:key='hash-8c07f894322d86705f9804d682a9ed6c9cd4be7a8fc6889d20b446e1d852fa8c'"
```

## Events

| Event name                | Included values and type                                                                           |
| ------------------------- | -------------------------------------------------------------------------------------------------- |
| Stake                     | token_address (String) , staker_address (Address) , requested_amount (U256) , staked_amount (U256) |
| PaidOut                   | token_address (String) , staker_address (Address) , amount (U256), reward (U256)                   |
| AddReward                 | reward_amount (U256),  withdrawable_amount (U256)                                                  |
| Refunded                  | token_address (String) , staker_address (Address) , amount (U256)                                  |

## Error Codes

| Code | Error                                               |
| ---- | --------------------------------------------------- |
| 1    | PermissionDenied                                    |
| 2    | WrongArguments                                      |
| 3    | NotRequiredStake                                    |
| 4    | AfterBadTiming                                      |
| 5    | BeforeBadTiming                                     |
| 6    | InvalidContext                                      |
| 7    | NegativeReward                                      |
| 8    | NegativeWithdrawableReward                          |
| 9    | NegativeAmount                                      |
| 10   | MissingContractPackageHash                          |
| 11   | InvalidContractPackageHash                          |
| 12   | InvalidContractHash                                 |
| 13   | WithdrawCheckErrorEarly                             |
| 14   | WithdrawCheckError                                  |
| 15   | NeitherAccountHashNorNeitherContractPackageHash     |
| 16   | NotAStaker                                          |
| 17   | ImmediateCallerAddressFail                          |
| 18   | NotStakingContractPackageHash                       |
| 19   | StakingEndsBeforeStakingStarts                      |
| 20   | WithdrawStartsStakingEnds                           |
| 21   | WithdrawEndsWithdrawStarts                          |
| 22   | StakingStartsNow                                    |
| 23   | CheckedSub                                          |
| 24   | GapBetweenStakingEndsWithdrawStarts                 |

## Contributing

If you would like to contribute to this repository, please fork the repository and create a new branch for your changes. Once you have made your changes, submit a pull request and we will review your changes.

Please ensure that your code follows the style and conventions used in the existing codebase, and that it passes all tests before submitting a pull request.

## License

The smart contracts in this repository are licensed under the [MIT License](https://opensource.org/licenses/MIT).
