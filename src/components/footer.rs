use yew::prelude::*;

use crate::AppContext;

#[function_component(Footer)]
pub fn footer() -> Html {
    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");

    fn handle_footer_content(app_context: AppContext) -> Html {
        let yew_link: Html = html! { <a href="https://yew.rs/" target="_blank" rel="noopener noreferrer">{ "Yew Framework" }</a> };
        let wasm_link: Html = html! { <a href="https://webassembly.org/" target="_blank" rel="noopener noreferrer">{ "WebAssembly" }</a> };

        match app_context.language.current {
            "de" => html! {
                    <>
                {  "Diese Website wurde in "} <strong>{"Rust"}</strong>{" mit dem " } { yew_link } { " programmiert und zu " } { wasm_link } { " kompiliert." }
                    </>
            },
            "eng" | _ => html! {
            <>
                {"This website was created in "} <strong>{"Rust"}</strong> {" utilizing the "} { yew_link } { " and is compiled to " } { wasm_link } { "." }
            </>
            },
        }
    }

    html! {
    <footer>
        { handle_footer_content(app_context) }
    </footer>
    }
}
