# Fennel Identity Pallet

The Fennel Identity pallet implements our concept for composable identities. Creating a new identity reserves an integer pointer on chain which is immutably connected back to your account. From there you can create multiple profiles for various contexts by connecting them back to that integer pointer.

Identity Traits create a way to map key-value pairs to an owned identity, effectively allowing applications to commit customizable profiles to the chain.
