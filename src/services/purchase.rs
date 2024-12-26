pub struct PurchaseManager {
    inventory_manager: Arc<InventoryManager>,
    reservation_manager: Arc<ReservationManager>,
}

impl PurchaseManager {
    pub async fn process_bulk_purchase(&self, orders: Vec<BulkOrderItem>) -> Result<PurchaseResult, Box<dyn std::error::Error>> {
        let mut successful_orders = Vec::new();
        let mut failed_orders = Vec::new();
        let mut reservations = Vec::new();

        // First phase: Try to reserve all items
        for order in &orders {
            match self.reservation_manager
                .create_reservation(order.product_id.clone(), order.quantity)
                .await
            {
                Ok(reservation_id) => {
                    reservations.push((order, reservation_id));
                }
                Err(_) => {
                    failed_orders.push(order.clone());
                }
            }
        }

        // Second phase: If all reservations successful, process purchases
        if failed_orders.is_empty() {
            for (order, _) in &reservations {
                match self.inventory_manager
                    .purchase_product(&order.product_id, order.quantity)
                    .await
                {
                    Ok(_) => successful_orders.push(order.clone()),
                    Err(_) => failed_orders.push(order.clone()),
                }
            }
        }

        // If any purchases failed, rollback all reservations
        if !failed_orders.is_empty() {
            for (order, _) in reservations {
                self.inventory_manager
                    .release_reservation(&order.product_id, order.quantity)
                    .await
                    .ok();
            }
        }

        Ok(PurchaseResult {
            successful_orders,
            failed_orders,
        })
    }
}