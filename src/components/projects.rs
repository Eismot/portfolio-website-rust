use yew::prelude::*;

use crate::components::svg::emojis::Robot;
use crate::AppContext;

#[function_component(Projects)]
pub fn projects() -> Html {
    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");

    fn handle_title(app_context: &AppContext) -> &'static str {
        match app_context.language.current {
            "de" => "Projekte",
            "eng" | _ => "Projects",
        }
    }

    // Share Advisor section
    fn handle_shad_title(app_context: &AppContext) -> &'static str {
        match app_context.language.current {
            "de" => "Share Advisor Backend und App",
            "eng" | _ => "Share Advisor Backend and mobile App",
        }
    }

    fn handle_shad_subtitle(app_context: &AppContext) -> &'static str {
        match app_context.language.current {
            "de" => "Real-time Voice & Video Chat App",
            "eng" | _ => "Real-time voice & video chat app",
        }
    }

    fn handle_shad_description(app_context: &AppContext) -> &'static str {
        match app_context.language.current {
            "de" => "Real-time Voice & Video Chat App",
            "eng" | _ => "Real-time voice & video chat app",
        }
    }

    fn handle_shad_link(app_context: &AppContext) -> &'static str {
        match app_context.language.current {
            "de" => "Lade die App herunter!",
            "eng" | _ => "Download the app!",
        }
    }

    html! {
    <>
        <h3 id="projects and experience">{ handle_title(&app_context) }{ " " }<Robot /></h3>
        // babbeln.app
        <div class="projects__project projects__project--babbeln">
        <section class="projects__project__info">
            <h4 class="projects__project__info__title">{ handle_shad_title(&app_context) }</h4>
            <h5 class="projects__project__info__subtitle">{ handle_shad_subtitle(&app_context) }</h5>
            <p class="projects__project__info__description">{ handle_shad_description(&app_context) }</p>
            <a href="https://shareadvisor.io/" target="_blank" rel="noopener noreferrer" class="projects__project__info__link margin-right">
            { handle_shad_link(&app_context) }
            </a>
            // <a href="https://gitlab.com/babbeln-client" target="_blank" rel="noopener noreferrer" class="projects__project__info__link--lighter">
            // { handle_see_code_link(&app_context) }
            // </a>
        </section>
        <img src="./assets/images/babbeln.png" alt="babbeln" class="projects__project__pic projects__project__pic--babbeln" loading="lazy"/>
        </div>
    </>
    }
}
