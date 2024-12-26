#[function_component(Chat)]
pub fn chat() -> Html {
    let messages = use_state(Vec::new);
    let message_input = use_state(String::new);

    let onsubmit = {
        let messages = messages.clone();
        let message_input = message_input.clone();
        
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            // Add message sending logic here
            message_input.set(String::new());
        })
    };

    html! {
        <div class="chat-container">
            <div class="messages">
                {for messages.iter().map(|msg| html! {
                    <div class="message">
                        <span class="username">{&msg.username}</span>
                        <span class="content">{&msg.message}</span>
                    </div>
                })}
            </div>
            <form onsubmit={onsubmit}>
                <input
                    type="text"
                    value={(*message_input).clone()}
                    onchange={let message_input = message_input.clone();
                        move |e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            message_input.set(input.value());
                        }
                    }
                />
                <button type="submit">{"Send"}</button>
            </form>
        </div>
    }
}