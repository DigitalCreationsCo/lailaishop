use std::error::Error;

pub async fn process_payment(amount: f64, seller_id: String) -> Result<(), Box<dyn Error>> {
    // Log the payment attempt
    log::info!(
        "Processing mock payment - Amount: ${:.2}, Seller ID: {}",
        amount,
        seller_id
    );

    // Simulate a brief processing delay
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    // Always return success
    Ok(())
}