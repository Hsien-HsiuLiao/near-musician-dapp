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


// collection, https://www.near-sdk.io/contract-structure/collections UnorderedMap, UnorderedSet and Vector
pub struct favSongs {
  songs: LookupMap<accountId, favsongs>
    }
    
//storage
    
//context, env


use near_sdk::{near_bindgen, env};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct StatusMessage {
    records: HashMap<AccountId, String>,
  //or songbyartist: <HashMap<AccountId, songlist>
}
pub struct songlist {
  songs: Vec<songs>
}
pub struct songs {
  name: String,
  price: u8
}

#[near_bindgen]
impl StatusMessage {
    pub fn set_status(&mut self, message: String) {
        let account_id = env::signer_account_id();
        self.records.insert(account_id, message);
    }

    pub fn get_status(&self, account_id: AccountId) -> Option<String> {
        self.records.get(&account_id).cloned()
    }
}
