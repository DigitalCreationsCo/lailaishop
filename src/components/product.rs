use yew::prelude::*;
use web_sys::{WebSocket, MessageEvent};
use wasm_bindgen::prelude::*;
use serde_json;
use types::InventoryUpdate;
use services::InventoryManager;

#[derive(Properties, Clone, PartialEq)]
pub struct ProductCardProps {
    pub product: Product,
}

#[function_component(ProductCard)]
pub fn product_card(props: &ProductCardProps) -> Html {
    let product = use_state(|| props.product.clone());
    let in_cart = use_state(|| 0u32);
    
    // Set up WebSocket connection for real-time inventory updates
    use_effect_with_deps(
        |(product,)| {
            let ws = WebSocket::new("ws://your-server/inventory").unwrap();
            let product_id = product.id.clone();
            
            let onmessage = Callback::from(move |e: MessageEvent| {
                let update: InventoryUpdate = serde_json::from_str(&e.data().as_string().unwrap()).unwrap();
                if update.product_id == product_id {
                    // Update local product state with new inventory data
                    product.set(Product {
                        stock: update.new_stock,
                        ..(*product).clone()
                    });
                }
            });
            
            ws.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
            
            || ws.close()
        },
        (product.clone(),),
    );

    let add_to_cart = {
        let product = product.clone();
        let in_cart = in_cart.clone();
        
        Callback::from(move |_| {
            let product_val = (*product).clone();
            let available = product_val.stock - product_val.reserved;
            
            if available > 0 {
                // Call reserve_product service
                spawn_local(async move {
                    let inventory_manager = use_context::<InventoryManager>().unwrap();
                    if inventory_manager.reserve_product(&product_val.id, 1).await.unwrap() {
                        in_cart.set(*in_cart + 1);
                    }
                });
            }
        })
    };

    html! {
        <div class="product-card">
            <h3>{&product.name}</h3>
            <p>{&product.description}</p>
            <p class="price">{"$"}{product.price}</p>
            <p class="stock">
                {format!("Available: {}", product.stock - product.reserved)}
            </p>
            <button 
                onclick={add_to_cart}
                disabled={product.stock - product.reserved == 0}
            >
                {if product.stock - product.reserved == 0 {
                    "Out of Stock"
                } else {
                    "Add to Cart"
                }}
            </button>
            if *in_cart > 0 {
                <p class="in-cart">{format!("In cart: {}", *in_cart)}</p>
            }
        </div>
    }
}
