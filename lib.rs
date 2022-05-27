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
