cracking timeless jewel

Goal:
Given Jewel Seed, Jewel Socket Id, Notable Id, Find the corresponding replacement notable index.

The Jewel Seed (and other information including Jewel Socket Id) is sent to the server. The server sends back a new jewel seed which is used in the PRNG to determine the replacement notable.

Two main challenges:
1. The function that determines the new jewel seed from the old jewel seed and jewel socket id is not known. There may be additional inputs that are not clear.
2. The PRNG seeding process seems to be more complex than just the two seeds. There may be additional seeds that are used for the PRNG.

Neither of the two are trivial.
