use reqwest::{self, Error};
use ring;
use ring::{digest, hmac, rand};
use hex::ToHex;
use serde_json::{self, Value};
use std::io::Read;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::str;
use structs::*;
use url::Url;

header! { (BittrexApiSign, "apisign") => [String] }

pub struct Client {
    api_key: String,
    api_secret: String
}

impl Client {
    pub fn new(api_key: &str, api_secret: &str) -> Result<Self, Error> {
        Ok(Self {
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string()
        })
    }

    pub fn get_balances(&self) -> Result<Option<Response>, serde_json::Error> {
        let url = Url::parse("https://bittrex.com/api/v1.1/account/getbalances").unwrap();
        let body: String = self.signed_get_request(url).unwrap();
        let balances: Response = serde_json::from_str(&body)?;

        Ok(
            Some(balances)
        )
    }

    pub fn get_balance(&self, currency: &str) -> Result<Option<Response>, serde_json::Error> {
        let url = Url::parse_with_params("https://bittrex.com/api/v1.1/account/getbalances",
                                         &[("currency", currency)]).unwrap();
        let body: String = self.signed_get_request(url).unwrap();
        let balances: Response = serde_json::from_str(&body)?;

        Ok(
            Some(balances)
        )
    }

    fn signed_get_request(&self, mut url: Url) -> Result<String, Error> {
        let now = SystemTime::now();
        let nonce = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let api_key = &self.api_key;
        url.query_pairs_mut()
            .append_pair("apikey", api_key)
            .append_pair("nonce", &format!("{:?}", nonce));
        let signed_url: &str = url.as_str();
        let signature = self.sign_request(signed_url).unwrap();
        let client = reqwest::Client::new()?;
        let mut request = client.get(signed_url)?
            .header(BittrexApiSign(signature))
            .build();

        let mut response = client.execute(request).unwrap();
        let mut body = String::new();
        response.read_to_string(&mut body);

        Ok(body)
    }

    fn sign_request(&self, url: &str) -> Result<String, ring::error::Unspecified> {
        let api_secret = &self.api_secret;
        let s_key = hmac::SigningKey::new(&digest::SHA512, &api_secret.as_bytes());
        let signature = hmac::sign(&s_key, url.as_bytes());
        let sig_ref = signature.as_ref();
        let sig_vec = sig_ref.to_vec();
        let signature_str = sig_vec.to_hex();

        Ok(signature_str)
    }
}
