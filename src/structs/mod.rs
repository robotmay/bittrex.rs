use decimal::*;
use serde;

#[derive(Serialize, Deserialize)]
pub struct Response {
    success: bool,
    message: String,
    result: ResponseValue
}

#[derive(Serialize, Deserialize)]
pub enum ResponseValue {
    Balance,
    BalanceCollection(Vec<Balance>),
    Currency,
    CurrencyCollection(Vec<Currency>),
}

#[derive(Serialize, Deserialize)]
pub struct Currency {
    Currency: String,
    CurrencyLong: String,
    MinConfirmation: d128,
    TxFee: d128,
    IsActive: bool,
    CoinType: String,
    BaseAddress: String
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
