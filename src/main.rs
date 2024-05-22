use ethers::core::k256::ecdsa::VerifyingKey;
use ethers::prelude::*;
use ethers::utils;
use ecies;
use dotenv::dotenv;
use std::env;

fn main() {
    // Until now we just do a Proof of concept of ecdsa_ecies encryption/decryption.

    // 1. Alice and Bob exchange their public address and  ecdsa key.
    // 2. Alice writes a message to Bob.
    // 3. Alice encrypts the message using ecies with ecdsa bobs public address
    // 4. Bob decripts the using his ecdsa private key.

    // Alice = account 1
    // Bob = account 2

    // ToDo. Add the signature section to verify the message

    // Step 1: Fetch ecdsa Private Keys (Ethereum Private Keys from .env)
    dotenv().ok();
    let private_key_1: String = env::var("PK0").unwrap().parse()
        .expect("error fetching the env variable PK0");
    let private_key_2: String = env::var("PK1").unwrap().parse()
        .expect("error fetching the env variable PK0");

    // Create a wallets from the private key
    let wallet_1: LocalWallet = private_key_1.parse().unwrap();
    let wallet_2: LocalWallet = private_key_2.parse().unwrap();
    
    // Private Keys as non zero scalar 
    println!("Signer1 Private Key is: {}", wallet_1.signer().as_nonzero_scalar());
    println!("Signer2 Private Key is: {}", wallet_2.signer().as_nonzero_scalar());

    // Access the full ecdsa public key 
    // NOTE: this is public key is the required for ECIES to provide
    // asymmetric encryption
    let ecdsa_public_key_1 = wallet_1.signer().verifying_key();
    let ecdsa_public_key_2 = wallet_2.signer().verifying_key();
    
    // Print the full uncompressed ECDSA public key
    println!("Full Public Key of signer 1: {}", ecdsa_public_key_1.to_encoded_point(false));
    println!("Full Public Key of signer 2: {}", ecdsa_public_key_2.to_encoded_point(false));
    
    // Print Full Public Address (without checksum) 32 Bytes keccak256 digest
    // Ethereum Wallet Conversion public key 1
    println!("Account1 eth address(full keccak256 digest): {:?}", public_key_to_full_address(ecdsa_public_key_1));
    println!("Account2 eth address(full keccak256 digest): {:?}", public_key_to_full_address(ecdsa_public_key_2));
    // TODO verify checksum 
    // https://github.com/ethereum/ercs/blob/master/ERCS/erc-55.md

    // Print Ethereum address (unsure if has checksum)
    let address_1 = utils::public_key_to_address(ecdsa_public_key_1);
    let address_2 = utils::public_key_to_address(ecdsa_public_key_2); 
    println!("Address 1: {:?}", address_1);
    println!("Address 2: {:?}", address_2);

    // Step 2: Encrypt a message using ECIES with public key of address2
    let message = b"Bob. Lets make Ethereum Cypherpunk Again!";
    let encrypted_message = ecies::encrypt(&ecdsa_public_key_2
                                           .to_encoded_point(false)
                                           .as_bytes()[1..], message)
        .expect("Encryption failed");
    println!("Encrypted message: {:?}", encrypted_message);
    
    let signer_2_private_key=wallet_2.signer().as_nonzero_scalar().to_bytes();
    // Step 4: Decrypt the message using private key of address2
    let decrypted_message = ecies::decrypt(&signer_2_private_key, &encrypted_message)
        .expect("Decryption failed");

    println!("Decrypted message: {:?}", std::str::from_utf8(&decrypted_message).unwrap());

}

pub fn public_key_to_full_address(pubkey: &VerifyingKey) -> H256 { 
    let pubkey = pubkey.to_encoded_point(false);
    let pubkey = &pubkey.as_bytes()[1..];
    assert_eq!(pubkey.len(), 64, "raw public key must be 64 bytes");
    let digest = utils::keccak256(pubkey);
    H256::from_slice(&digest)
}

//ToDo
//pub fn ecdsa_pk_to_ecies(pubkey: &VerifyingKey) -> [u8] { 
//}

