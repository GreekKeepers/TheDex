pub mod errors;
pub mod models;

use std::{collections::HashMap, sync::Arc};

use base64::{engine::general_purpose::STANDARD, Engine as _};
use hmac::{Hmac, Mac};
use models::Price;
use reqwest::Client;
use sha2::Sha512;
use tokio::sync::RwLock;

type HmacSha512 = Hmac<Sha512>;

const BASE_URL: &str = "https://app.thedex.cloud";

#[derive(Clone)]
pub struct TheDex {
    api_key: String,
    api_secret: String,
    last_requested: Arc<RwLock<u64>>,
    prices: Arc<RwLock<Vec<models::Price>>>,
    currencies: Arc<RwLock<Option<models::Currencies>>>,
}

impl TheDex {
    pub async fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_secret,
            api_key,
            prices: Arc::new(RwLock::new(Vec::with_capacity(0))),
            last_requested: Default::default(),
            currencies: Default::default(),
        }
    }

    async fn make_signed_request(
        &self,
        request: Option<models::Request>,
        path: &'static str,
        nonce: u64,
    ) -> Result<String, errors::Error> {
        let mut hashmap_serialized: HashMap<String, serde_json::Value> = if let Some(req) = request
        {
            serde_json::from_value(
                serde_json::to_value(&req).expect("Serialization to value failed"),
            )
            .unwrap()
        } else {
            Default::default()
        };

        hashmap_serialized.insert(
            String::from("request"),
            serde_json::Value::String(String::from(path)),
        );
        hashmap_serialized.insert(
            String::from("nonce"),
            serde_json::Value::String(nonce.to_string()),
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

        // let deserialized_res: Value =
        //     serde_json::from_str(&res).map_err(|err| errors::Error::SerdeError(err, res))?;

        Ok(res)
    }

    pub async fn create_quick_invoice(
        &self,
        request: models::CreateQuickInvoice,
        nonce: u64,
    ) -> Result<models::InvoiceCreateQuickResponse, errors::Error> {
        let response = self
            .make_signed_request(
                Some(models::Request::CreateQuickInvoice(request)),
                "/api/v1/invoices/create/quick",
                nonce,
            )
            .await?;
        if let Ok(response) = serde_json::from_str(&response) {
            Ok(response)
        } else {
            Err(errors::Error::UnexpectedResponse(response))
        }
    }

    pub async fn prices(
        &mut self,
        nonce: u64,
    ) -> Result<Arc<tokio::sync::RwLock<Vec<Price>>>, errors::Error> {
        if chrono::Utc::now().timestamp_millis() as u64 - *self.last_requested.read().await < 60000
        {
            return Ok(self.prices.clone());
        }
        let response = self
            .make_signed_request(None, "/api/v1/info/user/currencies/crypto", nonce)
            .await?;
        if let Ok(response) = serde_json::from_str::<Vec<Price>>(&response) {
            let mut locked = self.last_requested.write().await;
            *locked = chrono::Utc::now().timestamp_millis() as u64;
            self.prices = Arc::new(RwLock::new(response));
            Ok(self.prices.clone())
        } else {
            Err(errors::Error::UnexpectedResponse(response))
        }
    }

    pub async fn currencies(&mut self, nonce: u64) -> Result<models::Currencies, errors::Error> {
        if let Some(cur) = self.currencies.read().await.as_ref() {
            return Ok(cur.clone());
        }
        let response = self
            .make_signed_request(None, "/api/v1/info/currencies", nonce)
            .await?;

        if let Ok(response) = serde_json::from_str::<models::Currencies>(&response) {
            let mut locked = self.currencies.write().await;
            locked.replace(response.clone());
            Ok(response)
        } else {
            Err(errors::Error::UnexpectedResponse(response))
        }
    }
}
