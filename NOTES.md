# TODO
* Write full test coverage for every function
* Include fuzz tests where appropriate
* Combine mine and mint events or really any looped event into a single batched event to make it smaller so it fits in the 2 KB limit
* Add some sort of NFT standard functions for viewing, transferring, etc.
* How do we plan to handle title and story info?

# DONE
* Add upgrade function during a beta period
* Set specific settings to instance vars and allow then to be modified by protocol owner address

# REJECTED
* Why store `Color` as `(miner, owner, color): amount` vs `(owner, color): (miner, amount)`
    * It would be nice to be able to look up how many of a specific color you have not just how much of a mined color
    * I think it's because it would make it very difficult to make and/or track royalty payments
* Switch to a model where royalty payments are claimed passively to save on reads and writes in a single transaction
    * Might should have the transfer payment be made though for the swap
    * This may not actually save anything though as payments still need to be made to the contract vault and then some sort of balance tracking would need to be configured likely reducing any savings other than maybe some cross contract calls to the fee token SAC
* Mechanic for making a "commitment to mint" where you transfer all the colors to a special minting contract where anyone can trigger the actual progressive mint
	* You are so dumb. "transfer all the colors". That's literally why we have progressive minting in the first place