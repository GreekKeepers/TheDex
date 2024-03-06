pub mod errors;
pub mod models;

use std::{collections::HashMap, sync::Arc};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::prelude::*;
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde_json::Value;
use sha2::Sha512;
use tokio::sync::RwLock;

type HmacSha512 = Hmac<Sha512>;

const BASE_URL: &str = "https://thedex.cloud";

#[derive(Clone)]
pub struct TheDex {
    api_key: String,
    api_secret: String,
    last_requested: Arc<RwLock<u64>>,
    prices: Arc<Vec<models::Price>>,
}

impl TheDex {
    pub async fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_secret,
            api_key,
            prices: Arc::new(Vec::with_capacity(0)),
            last_requested: Default::default(),
        }
    }

    async fn make_signed_request(
        &self,
        request: Option<models::Request>,
        path: &'static str,
        nonce: u64,
    ) -> Result<models::Response, errors::Error> {
        let raw_request = if let Some(req) = request {
            serde_json::to_value(&req).expect("Serialization to value failed")
        } else {
            Value::String("".into())
        };
        let mut hashmap_serialized: HashMap<String, serde_json::Value> =
            serde_json::from_value(raw_request).unwrap();

        hashmap_serialized.insert(
            String::from("request"),
            serde_json::Value::String(String::from(path)),
        );
        hashmap_serialized.insert(
            String::from("nonce"),
            serde_json::Value::Number(nonce.into()),
        );

        let serialized_request = serde_json::to_string(&hashmap_serialized).unwrap();

        let payload = STANDARD.encode(serialized_request.as_bytes());

        let mut mac = HmacSha512::new_from_slice(self.api_secret.as_bytes())
            .expect("HMAC can take key of any size");

        mac.update(payload.as_bytes());
        let signature_bytes = mac.finalize().into_bytes();
        let mut signature = String::new();
        for byte_entry in &signature_bytes {
            let hex_byte = format!("{:02x}", byte_entry);
            signature.push_str(&hex_byte);
        }

        let complete_url = format!("{}{}", BASE_URL, path);

        let client = Client::new();

        let res = client
            .post(complete_url)
            .body(serialized_request)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header("X-EX-APIKEY", &self.api_key)
            .header("X-EX-PAYLOAD", payload)
            .header("X-EX-SIGNATURE", signature)
            .send()
            .await
            .map_err(errors::Error::RequestError)?
            .text()
            .await
            .map_err(errors::Error::RequestError)?;

        let deserialized_res: models::Response =
            serde_json::from_str(&res).map_err(|err| errors::Error::SerdeError(err, res))?;

        Ok(deserialized_res)
    }

    pub async fn create_invoice(
        &self,
        request: models::CreateInvoice,
        nonce: u64,
    ) -> Result<models::InvoiceCreateResponse, errors::Error> {
        let response = self
            .make_signed_request(
                Some(models::Request::CreateInvoice(request)),
                "/api/v1/invoices/create",
                nonce,
            )
            .await?;
        if let models::Response::InvoiceCreateResponse(response) = response {
            Ok(response)
        } else {
            Err(errors::Error::UnexpectedResponse(response))
        }
    }

    pub async fn prices(&mut self, nonce: u64) -> Result<&Vec<models::Price>, errors::Error> {
        if chrono::Utc::now().timestamp_millis() as u64 - *self.last_requested.read().await < 60000
        {
            return Ok(&self.prices);
        }
        let response = self
            .make_signed_request(None, "/api/v1/info/user/currencies/crypto", nonce)
            .await?;
        if let models::Response::Prices(response) = response {
            let mut locked = self.last_requested.write().await;
            *locked = chrono::Utc::now().timestamp_millis() as u64;
            self.prices = Arc::new(response);
            Ok(&self.prices)
        } else {
            Err(errors::Error::UnexpectedResponse(response))
        }
    }
}
