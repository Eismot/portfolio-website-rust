use yew::prelude::*;

use crate::AppContext;
use crate::LanguageAction;
use crate::LanguageState;
use crate::ThemeAction;

use crate::components::svg::{
    flags::{America, Germany, Globe},
    themes::{Dark, Light},
};

#[function_component(Nav)]
pub fn nav() -> Html {
    let app_context: AppContext = use_context::<AppContext>().expect("No AppContext found!");

    let show_lang_dropdown: UseStateHandle<bool> = use_state(|| false);

    let cycle_theme = {
        let app_context = app_context.clone();
        let current_theme: &str = app_context.theme.current.clone();
        let current_theme_index: usize = match app_context
            .theme_cycle
            .iter()
            .position(|x: &&str| x == &current_theme)
        {
            Some(i) => i,
            None => 0,
        };
        let next_theme: &str = match app_context.theme_cycle.iter().nth(current_theme_index + 1) {
            Some(nt) => nt,
            None => "light",
        };
        Callback::from(move |_| match next_theme {
            "light" => app_context.theme.dispatch(ThemeAction::Light),
            "dark" | _ => app_context.theme.dispatch(ThemeAction::Dark),
        })
    };

    fn handle_nav_icon(app_context: &AppContext) -> &'static str {
        match app_context.theme.current {
            "light" => "./assets/images/logo.png",
            "dark" | _ => "./assets/images/logo_dark.png",
        }
    }

    fn handle_theme_icon(app_context: AppContext) -> Html {
        match app_context.theme.current {
            "light" => html! {<Light />},
            "dark" | _ => html! {<Dark />},
        }
    }

    let toggle_lang_dropdown = {
        let show_lang_dropdown: UseStateHandle<bool> = show_lang_dropdown.clone();
        Callback::from(move |_| show_lang_dropdown.set(!*show_lang_dropdown))
    };

    fn change_language(
        next_lang: &'static str,
        lang_state: UseReducerHandle<LanguageState>,
    ) -> Callback<MouseEvent> {
        Callback::from(move |_| match next_lang {
            "de" => lang_state.dispatch(LanguageAction::German),
            "eng" | _ => lang_state.dispatch(LanguageAction::English),
        })
    }

    fn get_projects_innerhtml(app_context: AppContext) -> &'static str {
        match app_context.language.current {
            "de" => "Projekte",
            "eng" | _ => "Projects",
        }
    }

    fn get_getintouch_innerhtml(app_context: AppContext) -> &'static str {
        match app_context.language.current {
            "de" => "Kontakt",
            "eng" | _ => "Contact",
        }
    }

    html! {
    <header class="navheader">

        /* Logo */
        <a href="/" class="logo">
        <img class="logo__img" src={ handle_nav_icon(&app_context) } alt="LOGO" loading="lazy"/>
        <span class="logo__text">{ "Thomas Siedenhans" }</span>
        </a>

        /* Navigation */
        <nav class="nav">
        <ul class="nav__items">
                /* Theme Switcher */
            <li class="nav__items__themeswitcher nav__leftside__item" onclick={ cycle_theme }>{ handle_theme_icon(app_context.clone()) }</li>

                /* Language Dropdown-Menu */
            <li class="nav__items__langswitcher" onclick={ toggle_lang_dropdown }><Globe /><span>{ app_context.language.current }</span>
            <ul
            class="nav__items__langswitcher__dropdown"
            style={ if *show_lang_dropdown { "display:block;" } else { "display:none;" } } >
                <li onclick={ change_language("eng", app_context.language.clone()) }><America />{ "English" }</li>
                <li onclick={ change_language("de", app_context.language.clone()) }><Germany />{ "Deutsch" }</li>
            </ul>
            </li>

                /* Link to Projects */
            <li class="nav__items__projects"><a href="#projects">{ get_projects_innerhtml(app_context.clone()) }</a></li>

                /* Link to Contact */
            <li class="nav__items__contact"><a href="#contact"><button>{ get_getintouch_innerhtml(app_context.clone()) }</button></a></li>
        </ul>
        </nav>
    </header>
    }
}
