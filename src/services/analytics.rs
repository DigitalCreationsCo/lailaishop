use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct SalesMetrics {
    total_revenue: f64,
    total_units_sold: u32,
    average_order_value: f64,
    stock_turnover_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryMetrics {
    low_stock_items: Vec<Product>,
    out_of_stock_items: Vec<Product>,
    stock_value: f64,
    reorder_suggestions: Vec<ReorderSuggestion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReorderSuggestion {
    product_id: String,
    suggested_quantity: u32,
    reason: String,
}

pub struct AnalyticsManager {
    inventory_manager: Arc<InventoryManager>,
    db_pool: Pool<Postgres>,
}

impl AnalyticsManager {
    pub async fn get_sales_metrics(&self, seller_id: &str, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Result<SalesMetrics> {
        let sales = sqlx::query!(
            r#"
            SELECT 
                SUM(quantity) as total_units,
                SUM(quantity * price) as total_revenue,
                COUNT(DISTINCT order_id) as order_count
            FROM sales
            WHERE seller_id = $1 AND created_at BETWEEN $2 AND $3
            "#,
            seller_id,
            start_date,
            end_date
        )
        .fetch_one(&self.db_pool)
        .await?;

        Ok(SalesMetrics {
            total_revenue: sales.total_revenue.unwrap_or(0.0),
            total_units_sold: sales.total_units.unwrap_or(0) as u32,
            average_order_value: sales.total_revenue.unwrap_or(0.0) / sales.order_count.unwrap_or(1) as f64,
            stock_turnover_rate: self.calculate_turnover_rate(seller_id).await?,
        })
    }

    pub async fn get_inventory_metrics(&self, seller_id: &str) -> Result<InventoryMetrics> {
        let products = self.inventory_manager.get_seller_products(seller_id).await?;
        
        let low_stock_threshold = 10; // Configurable
        let low_stock_items: Vec<Product> = products
            .iter()
            .filter(|p| p.stock < low_stock_threshold && p.stock > 0)
            .cloned()
            .collect();

        let out_of_stock_items: Vec<Product> = products
            .iter()
            .filter(|p| p.stock == 0)
            .cloned()
            .collect();

        let stock_value = products
            .iter()
            .map(|p| p.price * p.stock as f64)
            .sum();

        Ok(InventoryMetrics {
            low_stock_items,
            out_of_stock_items,
            stock_value,
            reorder_suggestions: self.generate_reorder_suggestions(&products).await?,
        })
    }

    async fn generate_reorder_suggestions(&self, products: &[Product]) -> Result<Vec<ReorderSuggestion>> {
        let mut suggestions = Vec::new();
        
        for product in products {
            let sales_velocity = self.calculate_sales_velocity(product.id.as_str()).await?;
            
            if product.stock < (sales_velocity * 7.0) as u32 { // Less than 7 days of inventory
                suggestions.push(ReorderSuggestion {
                    product_id: product.id.clone(),
                    suggested_quantity: (sales_velocity * 14.0) as u32, // 2 weeks of inventory
                    reason: "Low stock based on sales velocity".to_string(),
                });
            }
        }
        
        Ok(suggestions)
    }

    async fn calculate_sales_velocity(&self, product_id: &str) -> Result<f64, Box<dyn std::error::Error>> {
        // Calculate average daily sales over the last 30 days
        let thirty_days_ago = Utc::now() - chrono::Duration::days(30);
        
        let sales = sqlx::query!(
            r#"
            SELECT SUM(quantity) as total_quantity
            FROM sales
            WHERE product_id = $1 
            AND created_at > $2
            "#,
            product_id,
            thirty_days_ago
        )
        .fetch_one(&self.db_pool)
        .await?;

        // Calculate average daily sales (total sales / 30 days)
        let total_quantity = sales.total_quantity.unwrap_or(0) as f64;
        let daily_velocity = total_quantity / 30.0;

        Ok(daily_velocity)
    }
}