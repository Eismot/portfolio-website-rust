use yew::prelude::*;

use crate::components::svg::emojis::Nerd;
use crate::AppContext;

#[function_component(Skills)]
pub fn skills() -> Html {
    let app_context = use_context::<AppContext>().expect("No AppContext not found!");

    fn handle_title(app_context: AppContext) -> &'static str {
        match app_context.language.current {
            "de" => "Meine Skills und Technologien",
            "eng" | _ => "My Skills and technologies",
        }
    }

    html! {
    <>
            <h3>{ handle_title(app_context) } { " " } <Nerd /> </h3>
              <div class="skills">
                <div class="skills__item">
                  <img src="./assets/images/tech-icons/js.png" alt="JavaScript" loading="lazy"/>
                  <span class="skills__tooltip">{ "JavaScript" }</span>
                </div>
                <div class="skills__item">
                  <img src="./assets/images/tech-icons/sass.png" alt="Sass" loading="lazy"/>
                  <span class="skills__tooltip">{ "Sass" }</span>
                </div>
            /*
                <div class="skills__item">
                  <img src="./assets/images/tech-icons/jquery.png" alt="jQuery" />
                  <span class="skills__tooltip">{ "jQuery" }</span>
                </div>
            */
                <div class="skills__item">
                  <img src="./assets/images/tech-icons/node.png" alt="Node.js" loading="lazy"/>
                  <span class="skills__tooltip">{ "Node.js" }</span>
                </div>
                <div class="skills__item">
                  <img src="./assets/images/tech-icons/react.png" alt="React.js" loading="lazy"/>
                  <span class="skills__tooltip">{ "React.js" }</span>
                </div>
                <div class="skills__item">
                  <img src="./assets/images/tech-icons/ts.png" alt="TypeScript" loading="lazy"/>
                  <span class="skills__tooltip">{ "TypeScript" }</span>
                </div>
                <div class="skills__item">
                  <img src="./assets/images/tech-icons/wasm.png" alt="WebAssembly" loading="lazy"/>
                  <span class="skills__tooltip">{ "WebAssembly" }</span>
                </div>
                <div class="skills__item">
                  <img src="./assets/images/tech-icons/rust.png" alt="Rust" loading="lazy"/>
                  <span class="skills__tooltip">{ "Rust" }</span>
                </div>
            /*
                <div class="skills__item">
                  <img src="./assets/images/tech-icons/ruby.png" alt="Ruby" loading="lazy"/>
                  <span class="skills__tooltip">{ "Ruby" }</span>
                </div>
            */
                <div class="skills__item">
                  <img src="./assets/images/tech-icons/linux.png" alt="GNU/Linux" loading="lazy"/>
                  <span class="skills__tooltip">{ "GNU/Linux" }</span>
                </div>
                <div class="skills__item">
                  <img src="./assets/images/tech-icons/git.png" alt="Git" loading="lazy"/>
                  <span class="skills__tooltip">{ "Git" }</span>
                </div>
                <div class="skills__item">
                  <img src="./assets/images/tech-icons/docker.png" alt="Docker" loading="lazy"/>
                  <span class="skills__tooltip">{ "Docker" }</span>
                </div>
                <div class="skills__item">
                  <img src="./assets/images/tech-icons/mysql.png" alt="MySQL" loading="lazy"/>
                  <span class="skills__tooltip">{ "MySQL" }</span>
                </div>
                <div class="skills__item">
                  <img src="./assets/images/tech-icons/mongo.png" alt="mongoDB" loading="lazy"/>
                  <span class="skills__tooltip">{ "mongoDB" }</span>
                </div>
              </div>
            </>
    }
}
