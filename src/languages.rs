use lazy_static::lazy_static;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Language {
    title: &'static str,
    excitement: u8,
    skill: u8,
}

lazy_static! {
    pub static ref LANGUAGES: Vec<&'static Language> = vec![
        &Language {
            title: "Rust",
            excitement: 95,
            skill: 70,
        },
        &Language {
            title: "Go",
            excitement: 80,
            skill: 85,
        },
        &Language {
            title: "Python",
            excitement: 60,
            skill: 80,
        },
        &Language {
            title: "Scrum",
            excitement: 80,
            skill: 80,
        },
        &Language {
            title: "Kubernetes",
            excitement: 80,
            skill: 80,
        },
        &Language {
            title: "Terraform",
            excitement: 75,
            skill: 75,
        },
        &Language {
            title: "SQL",
            excitement: 40,
            skill: 80,
        },
        &Language {
            title: "C#",
            excitement: 50,
            skill: 30,
        },
        &Language {
            title: "C++",
            excitement: 40,
            skill: 70,
        },
        &Language {
            title: "Kotlin",
            excitement: 70,
            skill: 70,
        },
        &Language {
            title: "Java",
            excitement: 20,
            skill: 70,
        },
        &Language {
            title: "HTML/CSS",
            excitement: 50,
            skill: 60,
        },
        &Language {
            title: "TypeScript",
            excitement: 65,
            skill: 60,
        },
        &Language {
            title: "JavaScript",
            excitement: 30,
            skill: 60,
        },
        &Language {
            title: "Vue.js",
            excitement: 60,
            skill: 60,
        },
    ];
}

fn sorted_languages() -> Vec<&'static Language> {
    let mut langs = LANGUAGES.clone();
    langs.sort_by(|a, b| b.excitement.cmp(&a.excitement));
    langs
}

#[function_component(Languages)]
pub fn languages() -> Html {
    html!(
        <div class="container">
            <h1 class="title is-1 has-text-centered">{ "Languages/Technologies/Skills" }</h1>
            <div class="card is-full-height has-background-primary-light">
                <div class="card-content">
                    { for sorted_languages().into_iter().map(|language| {
                        html!(<LanguageComponent ..language.clone() />)
                    })}
                 </div>
            </div>
        </div>
    )
}

#[function_component(LanguageComponent)]
pub fn language(language: &Language) -> Html {
    html!(
        <div class="columns is-centered is-vcentered is-6">
            <div class="column is-2 is-full-height">
                <h1 class="title is-4">{ language.title }</h1>
            </div>
            <div class="column is-5 is-flex is-align-items-center">
                <span class="icon is-large has-text-danger mr-1">
                    <i class="mdi mdi-36px mdi-heart"></i>
                </span>
                <progress class="progress is-medium is-danger" value={format!("{}", language.excitement)} max="100">{ format!("{}%", language.excitement) }</progress>
            </div>
            <div class="column is-5 is-flex is-align-items-center">
                <span class="icon is-large has-text-success mr-1">
                    <i class="mdi mdi-36px mdi-lightbulb-on"></i>
                </span>
                <progress class="progress is-medium is-success" value={format!("{}", language.skill)} max="100">{ format!("{}%", language.skill) }</progress>
            </div>
        </div>
    )
}
