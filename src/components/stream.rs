use web_sys::{HtmlVideoElement, MediaStreamConstraints};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(LiveStream)]
pub fn live_stream() -> Html {
    let video_ref = use_node_ref();

    use_effect_with_deps(
        move |video_ref| {
            let video = video_ref.cast::<HtmlVideoElement>().unwrap();
            
            spawn_local(async move {
                let window = web_sys::window().unwrap();
                let navigator = window.navigator();
                let media_devices = navigator.media_devices().unwrap();
                
                let constraints = MediaStreamConstraints::new();
                constraints.set_video(&JsValue::from(true));
                constraints.set_audio(&JsValue::from(true));
                
                if let Err(err) = async {
                    let promise = media_devices.get_user_media_with_constraints(&constraints)?;
                    let stream = wasm_bindgen_futures::JsFuture::from(promise).await?;
                    
                    video.set_src_object(Some(&stream));
                    let _ = video.play();
                    Ok::<(), JsValue>(())
                }.await {
                    log::error!("Error accessing media devices: {:?}", err);
                }
            });
            
            || ()
        },
        video_ref.clone(),
    );

    html! {
        <video ref={video_ref} autoplay=true />
    }
}