# Fennel Web-of-Trust Pallet

Support for web-of-trust actions between accounts on-chain.

## Issue Trust

Given an address, projects a trust connection between your account and the address specified.

## Remove Trust

Cancels a pre-existing trust issuance.

## Request Trust

Projects a request to the given address asking them to issue trust to your address.

## Cancel Trust Request

Cancels a pre-existing trust request.

## Revoke Trust

Broadcasts a transaction specifically announcing distrust for the given address.

## Remove Revoked Trust

Cancels a distrust signal already transmitted to the chain.

## Set Trust Parameter

For applications that use weighted transitive trust, use this extrinsic to announce key-value parameters used in the final weighting function.
