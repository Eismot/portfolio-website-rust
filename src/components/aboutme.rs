use yew::prelude::*;

use crate::components::svg::{emojis::Mail, flags::Germany};
use crate::AppContext;

#[function_component(Aboutme)]
pub fn aboutme() -> Html {
    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");

    fn handle_aboutme_content(app_context: AppContext) -> Html {
        match app_context.language.current {
            "de" => html! {
            <>
              <p>{ "Ich bin leidenschaftlicher Softwareentwickler/-consultant und Fintech Enthusiast aus München" }<Germany class="svg-text" /></p>
              <p>
                { "Mein Fokus liegt auf der Entwicklung und dem Deployment von hochqualitativer Software - in der Cloud oder On-Prem. " }
                { "Besonders liegen mir Data- und Machine Learning Engineering Projekte am Herzen." }
              </p>
              <p>
                <strong>{ "Aktuell arbeite ich mit folgenden Programmiersprachen:" } </strong> // <Heart class="svg-text svg-heart" />
                <br/><strong> { "Python" } </strong>
                { " (pandas, numpy, tensorflow, scikit-learn)" }
                <br/><strong> { "TypeScript" } </strong>
                { " (Vue.js, React.js, Node.js, Nest.js)" }
                <br/><strong> { "Java" } </strong>
                { " (Springboot, Hibernate, JPA, Maven)" }
                <br/><strong> { "Rust" } </strong>
                { " (Tokio, Yew)" }
                <br/>
                <br/>
                { "Interessiert and einem technischen oder wirtschaftlichen Austausch?"}
                <br/>
                {"Egal ob über spannende neue Technologien, oder wie Software dir und deiner Branche helfen könnte. Schreib mir einfach eine " }
                <a href="mailto:thomas.siedenhans@shareadvisor.io">{ "E-Mail" }<Mail class="svg-text" /></a>
              </p>
            </>
            },
            "eng" | _ => html! {
            <>
              <p>{ "I am a passionate software developer/consultant and fintech enthusiast from Munich, Germany." }<Germany class="svg-text" /></p>
              <p>
                { "My focus is on the development and deployment of high quality software - in the Cloud or On-Premises. " }
                { "Data and machine learning engineering projects are especially close to my heart." }
              </p>
              <p>
                <strong>{ "Currently I work with the following programming languages:" } </strong> // <Heart class="svg-text svg-heart" />
                <br/><strong> { "Python" } </strong>
                { " (pandas, numpy, tensorflow, scikit-learn)" }
                <br/><strong> { "TypeScript" } </strong>
                { " (Vue.js, React.js, Node.js, Nest.js)" }
                <br/><strong> { "Java" } </strong>
                { " (Springboot, Hibernate, JPA, Maven)" }
                <br/><strong> { "Rust" } </strong>
                { " (Tokio, Yew)" }
                <br/>
                <br/>
                { "Interested in a technical or economic exchange?" }
                <br/>
                { "Whether it's about exciting new technologies, or how software could help you and your industry. Just write me a " }
                <a href="mailto:thomas.siedenhans@shareadvisor.io">{ "E-Mail" }<Mail class="svg-text" /></a>
              </p>
            </>
            },
        }
    }

    html! {
    <div class="aboutme">
            <div class="aboutme__description">
                <div class="aboutme__description__text">{ handle_aboutme_content(app_context) }</div>
            </div>
        </div>
    }
}
