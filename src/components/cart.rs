#[function_component(Cart)]
pub fn cart() -> Html {
    let cart_items = use_state(HashMap::new);
    
    let checkout = {
        let cart_items = cart_items.clone();
        
        Callback::from(move |_| {
            let items = (*cart_items).clone();
            spawn_local(async move {
                let inventory_manager = use_context::<InventoryManager>().unwrap();
                
                // Attempt to purchase all items in cart
                let mut success = true;
                for (product_id, quantity) in items.iter() {
                    if !inventory_manager.purchase_product(product_id, *quantity).await.unwrap() {
                        success = false;
                        break;
                    }
                }
                
                if success {
                    // Process payment
                    // Clear cart
                    cart_items.set(HashMap::new());
                } else {
                    // Show error message to user
                    // Release any successful reservations
                    for (product_id, quantity) in items.iter() {
                        let _ = inventory_manager.release_reservation(product_id, *quantity).await;
                    }
                }
            });
        })
    };

    html! {
        <div class="cart">
            {for cart_items.iter().map(|(product_id, quantity)| html! {
                <CartItem 
                    product_id={product_id.clone()}
                    quantity={*quantity}
                />
            })}
            <button onclick={checkout}>{"Checkout"}</button>
        </div>
    }
}