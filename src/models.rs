use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Request {
    CreateInvoice(CreateInvoice),
}

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreateInvoice {
    #[serde(with = "rust_decimal::serde::str")]
    pub amount: Decimal,
    pub currency: String,
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

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum Response {
    Invoice(Invoice),
    InvoiceCreateResponse(InvoiceCreateResponse),
}

#[derive(Serialize_repr, Deserialize_repr)]
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

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceCreateResponse {
    pub invoice_id: Option<String>,
    pub merchant_id: String,
    pub client_id: Option<String>,
    pub order_id: Option<String>,
    pub create_date: String,
    pub modified_date: String,
    pub status: InvoiceStatus,
    pub pay_url: String,
}

#[derive(Deserialize, Serialize)]
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
