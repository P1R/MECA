# MECA

Make Ethereum Cypherpunk Again: Asymmetric Encryption using an EOA, by using the ECIES on ECDSA

## Use Case Example
1. Bob and Alice publish their ecdsa key paired with his public address in a smart contract
2. Alice gets Bob public address and  ecdsa key.
3. Alice writes a message to Bob.
4. Alice encrypts the message using ecies with ecdsa bobs public address
5. Alice stores the encrypted message in IPFS and gets an CID.
6. Alice signs an push a transaction with the encrypted message CID using Bobs public ethereum address
7. Bob verifies signature and get the CID file fetching it from IPFS.
8. Bob decripts the CID fetche message using his ecdsa private key.

## References

1. https://eips.ethereum.org/EIPS/eip-5630
2. https://docs.rs/ecies/0.2.7/ecies/index.html
3. https://ethereum.stackexchange.com/questions/56253/where-does-ethereum-store-eoas-and-public-keys
