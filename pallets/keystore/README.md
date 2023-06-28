# Fennel Keystore Announcements Pallet

Fennel Protocol includes features for broadcasting public keys.

Traditional keyservers tend to be a bit disjoint and difficult to keep track of - the same key might be present on dozens of servers, with no real certainty on whether updates or revocations are present across locations.

Fennel's keystore pallet provides a series of specialized on-chain signals for announcing a public key by its fingerprint and a URI for the location it's been uploaded to.

## Announce Key

RSA encryption/signing keys should be announced through this extrinsic by fingerprint and location. These are encoded as vectors of u8s.

## Revoke Key

This extrinsic takes a key fingerprint and marks the key as revoked from circulation.

## Issue Encryption Key

Used for 32-byte public keys generated for use in Elliptic-curve Diffie-Hellman. 
