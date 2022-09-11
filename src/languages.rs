use lazy_static::lazy_static;
use yew::prelude::*;

#[derive(Clone)]
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
        <>
        <h1 class="title">{ "Languages/Technologies/Skills" }</h1>
        { for sorted_languages().iter().map(|language| {
            html!(
                <div class="columns is-centered is-vcentered is-6">
                    <div class="column is-1 is-full-height">
                        <h1 class="title is-4">{ language.title }</h1>
                    </div>
                    <div class="column is-4 is-flex is-align-items-center">
                        <span class="icon is-large has-text-danger mr-1">
                            <i class="mdi mdi-36px mdi-heart"></i>
                        </span>
                        <progress class="progress is-medium is-danger" value={format!("{}", language.excitement)} max="100">{ format!("{}%", language.excitement) }</progress>
                    </div>
                    <div class="column is-4 is-flex is-align-items-center">
                        <span class="icon is-large has-text-success mr-1">
                            <i class="mdi mdi-36px mdi-lightbulb-on"></i>
                        </span>
                        <progress class="progress is-medium is-success" value={format!("{}", language.skill)} max="100">{ format!("{}%", language.skill) }</progress>
                    </div>
                </div>
            )
        })}
        </>
    )
}
