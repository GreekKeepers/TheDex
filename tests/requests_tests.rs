use rust_decimal::Decimal;
use thedex::{self, models::CreateQuickInvoice};

const API_KEY: &str = "";
const SECRET_KEY: &str = "";

#[tokio::test]
pub async fn test_prices() {
    let mut dex = thedex::TheDex::new(API_KEY.into(), SECRET_KEY.into()).await;

    let result = dex
        .prices(chrono::Utc::now().timestamp_millis() as u64)
        .await;

    println!("{:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
pub async fn test_currencies() {
    let mut dex = thedex::TheDex::new(API_KEY.into(), SECRET_KEY.into()).await;

    let result = dex
        .currencies(chrono::Utc::now().timestamp_millis() as u64)
        .await;

    println!("{:?}", result);
    assert!(result.is_ok());
}

#[tokio::test]
pub async fn test_create_invoice() {
    let dex = thedex::TheDex::new(API_KEY.into(), SECRET_KEY.into()).await;

    let result = dex
        .create_quick_invoice(
            CreateQuickInvoice {
                amount: Decimal::from_str_exact("0.00007500").unwrap(),
                pay_currency: "BTC_BITCOIN".into(),
                merchant_id: "LDB3LVD7".into(),
                order_id: Some("123".into()),
                email: None,
                client_id: Some("Client".into()),
                title: Some("Test".into()),
                description: Some("Test desc".into()),
                recalculation: Some(true),
                needs_email_confirmation: Some(false),
                success_url: None,
                failure_url: None,
                callback_url: Some("https://game.greekkeepers.io/".into()),
            },
            chrono::Utc::now().timestamp_millis() as u64,
        )
        .await;

    println!("{:?}", result);
    assert!(result.is_ok());
}
