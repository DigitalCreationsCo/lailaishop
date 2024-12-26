#[function_component(SellerStore)]
pub fn seller_store() -> Html {
    let products = use_state(Vec::new);

    html! {
        <div class="store-container">
            <div class="stream-section">
                <LiveStream />
                <Chat />
            </div>
            <div class="products-section">
                {for products.iter().map(|product| html! {
                    <div class="product-card">
                        <h3>{&product.name}</h3>
                        <p>{&product.description}</p>
                        <p class="price">{"$"}{product.price}</p>
                        <button onclick={let product = product.clone();
                            move |_| {
                                // Add to cart logic
                            }
                        }>
                            {"Add to Cart"}
                        </button>
                    </div>
                })}
            </div>
        </div>
    }
}