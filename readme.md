# **on-chain DApp**
Solana on-chain program allows you to leave and update reviews.

## **Technologies Used**
![rust Version](https://img.shields.io/badge/rust-nightly%20-purple)
![solana-program Version](https://img.shields.io/badge/solana-program-1.17.7%20-yellow)
![borsh Version](https://img.shields.io/badge/borsh-1.2.1%20-yellow)


## **Instalation**

### **Prerequisites**
```bash
rustup install nightly
```
```bash
rustup override set nightly
```
```bash
cargo clean && cargo build-sbf --verbose
```
```bash
solana config set --keypair /home/user/.config/solana/id.json
```
```bash
solana balance
```
```bash
solana program deploy target/sbf-solana-solana/release/review_dapp.so
```
```bash
solana program show <PROGRAM_ID>
```


## **License**
This project is licensed under the MIT License - see the [LICENSE](https://github.com/chemyl/review_dapp/blob/master/LICENSE) file for details.
