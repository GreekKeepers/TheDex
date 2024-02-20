pub mod errors;
pub mod models;

use std::collections::HashMap;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use hmac::{Hmac, Mac};
use reqwest::Client;
use sha2::Sha512;

type HmacSha512 = Hmac<Sha512>;

const BASE_URL: &str = "https://thedex.cloud";

#[derive(Clone)]
pub struct TheDex {
    api_key: String,
    api_secret: String,
}

impl TheDex {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            api_secret,
            api_key,
        }
    }

    async fn make_signed_request(
        &self,
        request: models::Request,
        path: &'static str,
        nonce: &str,
    ) -> Result<models::Response, errors::Error> {
        let raw_request = serde_json::to_value(&request).expect("Serialization to value failed");
        let mut hashmap_serialized: HashMap<String, serde_json::Value> =
            serde_json::from_value(raw_request).unwrap();

        hashmap_serialized.insert(
            String::from("request"),
            serde_json::Value::String(String::from(path)),
        );
        hashmap_serialized.insert(
            String::from("nonce"),
            serde_json::Value::String(String::from(nonce)),
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
            serde_json::from_str(&res).map_err(errors::Error::SerdeError)?;

        Ok(deserialized_res)
    }

    pub async fn create_invoice(
        &self,
        request: models::CreateInvoice,
        nonce: &str,
    ) -> Result<models::Response, errors::Error> {
        self.make_signed_request(
            models::Request::CreateInvoice(request),
            "/api/v1/invoices/create",
            nonce,
        )
        .await
    }
}
