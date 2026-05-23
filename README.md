cracking timeless jewel

Goal:
Given Jewel Seed, Jewel Socket Id, Notable Id, Find the corresponding replacement notable index.

The Jewel Seed (and other information including Jewel Socket Id) is sent to the server. The server sends back a new jewel seed which is used in the PRNG to determine the replacement notable.

Two main challenges:
1. The function that determines the new jewel seed from the old jewel seed and jewel socket id is not known. There may be additional inputs that are not clear.
2. The PRNG seeding process seems to be more complex than just the two seeds. There may be additional seeds that are used for the PRNG.

Neither of the two are trivial.

Research on PRNG seeding:
Changes for POE2 from the POE 1 version. These have been confirmed via Ghidra:
1. The PRNG is now seeded with seven seeds instead of two
   1. The first seed is a lookup value (likely related to the notable id)
   2. The second seed is the jewel seed. This is the one printed in the debug log NOT the one displayed on the jewel.The jewel seed itself is constrainted to a uint16. (Though it is zero-padded and cast to a uint32 in the PRNG)
   3. The third seed is some uint32 that I haven't figured out yet.
2. The PRNG has _slightly_ different initial parameters. See `lib.rs` for the new initial parameters.


