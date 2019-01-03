Constant Time Survey Notes
--------------------------

## What are our goals?

* identify places in the internal types and algorithms that final/intermediate secret values are vulnerable to timing attacks

## What are our ANTI-goals?

* not concerned with the public API

List of files surveyed:

* bitrepr
* byte_decoder
* curve - N/A
* ed25519 - mostly comes down to: if dalek is constant time we're good. Otherwise, we aren't!
* field - N/A: constants and traits only
* fp
* fp2elem
* fp6elem
* fp12elem
* hashable
* homogeneouspoint
* macros
* mod
* pairing
* rand_bytes - we should think more about rand number generation
* schnorr - from implementations should be scrutinized
* sha256
* api - oh Lord, validation
* nonemptyvec - not CT. Is fine?

General Questions

* do validation checks make things not constant time?
* should we mark functions that aren't constant time in some way?