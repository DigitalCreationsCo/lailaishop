use std::sync::Arc;
use std::error::Error;
use tokio;
use wasm_bindgen_futures;;
use anyhow::{Result, anyhow};

pub struct InventoryManager {
    products: Arc<RwLock<HashMap<String, Product>>>,
    ws_clients: Arc<RwLock<HashMap<String, WebSocketSender>>>,
}

impl InventoryManager {
    pub async fn reserve_product(&self, product_id: &str, quantity: u32) -> Result<bool> {
        let mut products = self.products.write().await;
        
        if let Some(product) = products.get_mut(product_id) {
            let available = product.stock - product.reserved;
            if available >= quantity {
                product.reserved += quantity;
                self.broadcast_update(product_id, product.clone()).await;
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub async fn purchase_product(&self, product_id: &str, quantity: u32) -> Result<bool> {
        let mut products = self.products.write().await;
        
        if let Some(product) = products.get_mut(product_id) {
            if product.reserved >= quantity && product.stock >= quantity {
                product.stock -= quantity;
                product.reserved -= quantity;
                self.broadcast_update(product_id, product.clone()).await;
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub async fn release_reservation(&self, product_id: &str, quantity: u32) -> Result<()> {
        let mut products = self.products.write().await;
        
        if let Some(product) = products.get_mut(product_id) {
            if product.reserved >= quantity {
                product.reserved -= quantity;
                self.broadcast_update(product_id, product.clone()).await;
            }
        }
        Ok(())
    }

    async fn broadcast_update(&self, product_id: &str, product: Product) {
        let update = InventoryUpdate {
            product_id: product_id.to_string(),
            new_stock: product.stock,
            operation: InventoryOperation::StockUpdate,
        };
        
        let clients = self.ws_clients.read().await;
        for sender in clients.values() {
            let _ = sender.send(serde_json::to_string(&update).unwrap());
        }
    }
}