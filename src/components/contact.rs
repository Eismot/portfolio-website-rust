use yew::prelude::*;

use crate::components::svg::{
    emojis::{Blushing, Mail},
    undraw::QuickChat,
};
use crate::AppContext;

#[function_component(Contact)]
pub fn contact() -> Html {
    let app_context = use_context::<AppContext>().expect("No AppContext found!");

    fn handle_title(app_context: AppContext) -> &'static str {
        match app_context.language.current {
            "de" => "Lass uns quatschen",
            "eng" | _ => "Let's chat",
        }
    }

    fn handle_contact_description(app_context: AppContext) -> Html {
        match app_context.language.current {
            "de" => html! {
            <>
                <p>{ "Interessiert and einem technischen oder wirtschaftlichen Austausch?"}</p>
                <p>
                {"Egal ob über spannende neue Technologien, oder wie Software dir und deiner Branche helfen könnte. Schreib mir einfach eine " }
                <a href="mailto:thomas.siedenhans@shareadvisor.io">{ "E-Mail" }<Mail class="svg-text" /></a>
                </p>
            </>
            },
            "eng" | _ => html! {
            <>
                <p>{ "Interested in a technical or economic exchange?" }</p>
                <p>
                {"Whether it's about exciting new technologies, or how software could help you and your industry. Just write me an " }
                <a href="mailto:thomas.siedenhans@shareadvisor.io">{ "E-Mail" }<Mail class="svg-text" /></a>
                </p>
            </>
            },
        }
    }

    html! {
    <>
        <h3 id="contact"> { handle_title(app_context.clone()) } { " " } <Blushing /> </h3>
        <div class="contact margin-top">
        <QuickChat class="contact__illustration"/>
        <div class="contact__paragraph"> { handle_contact_description(app_context) } </div>
        </div>
        </>
    }
}
