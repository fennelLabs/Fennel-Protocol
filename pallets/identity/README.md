# Fennel Identity Pallet

The Fennel Identity pallet implements our concept for composable identities. Creating a new identity reserves an integer pointer on chain which is immutably connected back to your account. From there you can create multiple profiles for various contexts by connecting them back to that integer pointer. This is meant to give one account the ability to create different identities for different contexts.

Identity Traits create a way to map key-value pairs to an owned identity, effectively allowing applications to commit customizable profiles to the chain.

## Create Identity

Creates a pointer to an identity claimed by the current account.

## Revoke Identity

Announces that an identity should no longer be considered valid.

## Add or Update Identity Trait

Announces a key/value pair attached to the given identity number.

## Remove Identity Trait

Deletes the given key for the given identity number.

## Sign for Identity

Issues a transaction that simply carries a signature on the identity number proving ownership.
