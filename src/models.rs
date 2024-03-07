use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Request {
    CreateQuickInvoice(CreateQuickInvoice),
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateQuickInvoice {
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal,
    pub pay_currency: String,
    pub merchant_id: String,
    pub order_id: Option<String>,
    pub email: Option<String>,
    pub client_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub recalculation: Option<bool>,
    pub needs_email_confirmation: Option<bool>,
    pub success_url: Option<String>,
    pub failure_url: Option<String>,
    pub callback_url: Option<String>,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum InvoiceStatus {
    Waiting = 0,
    PendingConfirm,
    Unpaid,
    Successful,
    Rejected,
    Underpaid,
    WaitingEmailConfirmation,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceCreateQuickResponse {
    pub invoice_id: Option<String>,
    pub merchant_id: String,
    pub client_id: Option<String>,
    pub order_id: Option<String>,
    pub create_date: String,
    pub modified_date: String,
    pub status: InvoiceStatus,
    pub pay_url: String,
    pub purse: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub amount_in_pay_currency: Decimal,
    pub pay_currency: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub invoice_id: Option<String>,
    pub merchant_id: String,
    pub order_id: Option<String>,
    pub client_id: Option<String>,
    pub status: InvoiceStatus,
    pub status_name: String,
    pub create_date: String,
    pub modified_date: String,
    pub expiration_date: String,
    pub expiration_date_in_milliseconds: u64,
    pub purse: String,
    pub currency: String,
    pub pay_currency: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub amount_in_pay_currency: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub paid_amount: Decimal,
    pub pay_url: String,
    pub callback_url: Option<String>,
    pub creation_way: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub merchant_commission: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub merchant_balance: Decimal,
    pub title: Option<String>,
    pub description: Option<String>,
    pub unique_user_id: Option<String>,
    #[serde(with = "rust_decimal::serde::str")]
    pub deposit_blockchain_fee: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub rate_with_commission: Decimal,
    pub tx_id: Vec<String>,
    pub failure_url: Option<String>,
    pub success_url: Option<String>,
    pub merchant_site_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Currencies {
    pub fiat_currencies: Vec<String>,
    pub pay_currencies: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    /// Ex: `BTC_BITCOIN`
    #[serde(with = "symbol")]
    pub monetary: Symbol,

    pub rates: Vec<Rate>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    /// Ex: `BTC_BITCOIN`
    pub short: String,

    pub full: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Rate {
    pub fiat_currency: String,
    #[serde(with = "rust_decimal::serde::str")]
    pub rate: Decimal,
}

pub mod date_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S%.3f";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}

pub mod symbol {
    use super::Symbol;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(symbol: &Symbol, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}_{}", symbol.short, symbol.full);
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Symbol, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let mut parts = s.split('_');
        let short = parts.next().unwrap();
        let full = if let Some(p) = parts.next() { p } else { short };
        Ok(Symbol {
            short: short.into(),
            full: full.into(),
        })
    }
}
