use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub is_seller: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub seller_id: String,
    pub stock: u32,
    pub reserved: u32,  // Track items in shopping carts
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventoryUpdate {
    pub product_id: String,
    pub new_stock: u32,
    pub operation: InventoryOperation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InventoryOperation {
    StockUpdate,
    Reserve,
    Purchase,
    ReleaseReservation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub user_id: String,
    pub username: String,
    pub message: String,
    pub timestamp: i64,
}