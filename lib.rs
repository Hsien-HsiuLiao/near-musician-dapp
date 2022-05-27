/* You must have a smart contract that you can deploy to NEAR TestNet and call
The contract interface must not be a copy of any examples or demos that you found.  If you are making a derivative work, please FORK the repo that inspired you or RISK losing your certification
The contract should use all of the following features of NEAR Protocol: 
Storage
Context
Persistent collections
assert statements
The contract should include scripts to drive it using either Bash and NEAR CLI or near-api-js in NodeJS files
The repo must include a loom.com video recording of your demo of 3-5 minutes length maximum
The repo must include sufficient documentation to standup your project after cloning locally
The repo must include some kind of explanation or overview of the intent of your project
SHOW ME SOME EXAMPLES
https://github.com/Learn-NEAR/NCD.L1.sample--thanks
https://github.com/Learn-NEAR/NCD.L1.sample--lottery
https://github.com/Learn-NEAR/NCD.L1.sample--nearly-neighbors
https://github.com/Learn-NEAR/NCD.L1.sample--meme-museum

*/
#[near-bindgen]
pub struct register {
  artistName: String, 
  account: accountid
}


// transition to different states
enum ticketPreSale{
  started,
  ended
}


// collection
pub struct favSongs {
  songs: LookupMap<accountId, favsongs>
    }
    
//storage
    
//context, env
