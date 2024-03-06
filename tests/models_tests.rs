use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};
use thedex::models;

#[test]
pub fn add_fields_to_requests() {
    let request = models::Request::CreateQuickInvoice(Default::default());
    let raw_request = serde_json::to_value(&request).expect("Serialization to value failed");

    let mut hashmap_serialized: HashMap<String, serde_json::Value> =
        serde_json::from_value(raw_request).unwrap();

    hashmap_serialized.insert(
        String::from("request"),
        serde_json::Value::String(String::from("/smth")),
    );
    hashmap_serialized.insert(
        String::from("nonce"),
        serde_json::Value::String(String::from("101010101")),
    );

    let result = serde_json::to_string(&hashmap_serialized).unwrap();

    println!("{:?}", result);
}

#[test]
pub fn date() {
    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%.3f";

    let s = "2024-02-20 13:46:30.035";

    let dt = NaiveDateTime::parse_from_str(&s, FORMAT).unwrap();
    let dt1 = DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc);

    println!("{:?} {:?}", dt, dt1);
}
