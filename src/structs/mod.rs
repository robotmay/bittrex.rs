use decimal::*;
use serde;

#[derive(Serialize, Deserialize)]
pub struct Balances {
    success: bool,
    message: String,
    result: Vec<Balance>
}

#[derive(Serialize, Deserialize)]
pub struct Balance {
   Currency: String,
   Balance: d128,
   Available: d128,
   Pending: d128,
   CryptoAddress: String,
   Requested: bool,
   Uuid: String
}
