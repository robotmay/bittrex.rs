use reqwest::{self, Error, Response};
use ring;
use ring::{digest, hmac, rand};
use hex::ToHex;
use serde_json::{self, Value};
use std::io::Read;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::str;

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

    pub fn get_balances(&self) -> Result<Value, serde_json::Error> {
        let url = "https://bittrex.com/api/v1.1/account/getbalances";
        let body: String = self.signed_get_request(url).unwrap();
        let balances = serde_json::from_str(&body);

        return balances;
    }

    fn signed_get_request(&self, url: &str) -> Result<String, Error> {
        let now = SystemTime::now();
        let nonce = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let api_key = &self.api_key;
        let s_url = format!("{url}?apikey={api_key}&nonce={nonce:?}", url=url, api_key=api_key, nonce=nonce);
        let signed_url: &str = &s_url;
        let signature = self.sign_request(signed_url).unwrap();
        let client = reqwest::Client::new().unwrap();
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
