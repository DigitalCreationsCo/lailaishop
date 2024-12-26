use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seller {
    pub id: String,
    pub name: String,
    pub email: String,
    pub notification_preferences: NotificationPreferences,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NotificationType {
    LowStock {
        product_id: String,
        current_stock: u32,
        threshold: u32,
    },
    OutOfStock {
        product_id: String,
    },
    ReorderSuggestion {
        product_id: String,
        suggested_quantity: u32,
        reason: String,
    },
    PaymentReceived {
        amount: f64,
        seller_id: String,
    },
    OrderConfirmation {
        order_id: String,
    },

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    pub low_stock_alerts: bool,
    pub payment_notifications: bool,
    pub order_updates: bool,
}

pub struct NotificationManager {
    db_pool: Arc<PgPool>,
}

impl NotificationManager {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }

    pub async fn get_all_sellers(&self) -> Result<Vec<Seller>, Box<dyn Error>> {
        let sellers = sqlx::query_as!(
            Seller,
            r#"
            SELECT 
                s.id,
                s.name,
                s.email,
                np.low_stock_alerts,
                np.payment_notifications,
                np.order_updates
            FROM sellers s
            LEFT JOIN notification_preferences np ON s.id = np.seller_id
            WHERE s.active = true
            "#
        )
        .fetch_all(&*self.db_pool)
        .await?;

        Ok(sellers)
    }
}