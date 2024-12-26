use tokio;
use std::error::Error;
use wasm_bindgen_futures;

pub struct ReservationManager {
    inventory_manager: Arc<InventoryManager>,
    timeout_duration: Duration,
}

impl ReservationManager {
    pub fn new(inventory_manager: Arc<InventoryManager>) -> Self {
        Self {
            inventory_manager,
            timeout_duration: Duration::from_secs(15 * 60), // 15 minutes
        }
    }

    pub async fn create_reservation(&self, product_id: String, quantity: u32) -> Result<String, Box<dyn std::error::Error>> {
        let reservation_id = uuid::Uuid::new_v4().to_string();
        
        if self.inventory_manager.reserve_product(&product_id, quantity).await? {
            let inventory_manager = self.inventory_manager.clone();
            let product_id_clone = product_id.clone();
            let timeout_duration = self.timeout_duration;

            tokio::spawn(async move {
                sleep(timeout_duration).await;
                inventory_manager.release_reservation(&product_id_clone, quantity).await.ok();
            });

            Ok(reservation_id)
        } else {
            Err(anyhow!("Unable to reserve product"))
        }
    }
}
