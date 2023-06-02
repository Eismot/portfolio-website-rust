use yew::prelude::*;

use crate::components::svg::emojis::Handshake;

#[function_component(Landing)]
pub fn landing() -> Html {
    html! {
    <>
        <h1 class="landing">
        { "Hi! I'm Thomas " } <Handshake class="wave" /> <br />
        { "I'm a fullstack, data and cloud engineer" } // <wbr /> { "stack de" } <wbr /> { "velo" } <wbr /> { "per" }
        </h1>

        <script src={ "./vendor/lottie_player_v157_bundle.js" }></script>

        <lottie-player
        class="lottie-landing"
        // This Lottiefile is licensed by CC so he must be attributed
        // title="Jignesh Gajjar @LottieFiles"
        src="./assets/lottie/coder_with_coffee_mug.json"
        background="transparent"
        speed="2"
        loop={ true }
        autoplay={ true } >
        </lottie-player>
    </>
    }
}
