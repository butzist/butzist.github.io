use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Skill {
    title: &'static str,
    excitement: u8,
    skill: u8,
}

thread_local! {
    pub static LANGUAGES: Vec<&'static Skill> = vec![
        &Skill {
            title: "Rust",
            excitement: 95,
            skill: 70,
        },
        &Skill {
            title: "Go",
            excitement: 80,
            skill: 85,
        },
        &Skill {
            title: "Python",
            excitement: 60,
            skill: 80,
        },
        &Skill {
            title: "Kubernetes",
            excitement: 80,
            skill: 80,
        },
        &Skill {
            title: "Terraform",
            excitement: 75,
            skill: 75,
        },
        &Skill {
            title: "SQL",
            excitement: 40,
            skill: 80,
        },
        &Skill {
            title: "C#",
            excitement: 50,
            skill: 30,
        },
        &Skill {
            title: "C++",
            excitement: 40,
            skill: 70,
        },
        &Skill {
            title: "Kotlin",
            excitement: 70,
            skill: 70,
        },
        &Skill {
            title: "Java",
            excitement: 20,
            skill: 70,
        },
        &Skill {
            title: "HTML/CSS",
            excitement: 50,
            skill: 60,
        },
        &Skill {
            title: "TypeScript",
            excitement: 65,
            skill: 60,
        },
        &Skill {
            title: "JavaScript",
            excitement: 30,
            skill: 60,
        },
        &Skill {
            title: "Vue.js",
            excitement: 60,
            skill: 60,
        },
        &Skill {
            title: "Scrum",
            excitement: 80,
            skill: 80,
        },
        &Skill {
            title: "Refactoring",
            excitement: 85,
            skill: 85,
        },
        &Skill {
            title: "Test Driven Development",
            excitement: 80,
            skill: 70,
        },
    ];
}

fn sorted_skills() -> Vec<&'static Skill> {
    let mut langs = LANGUAGES.with(|l| l.clone());
    langs.sort_by(|a, b| b.excitement.cmp(&a.excitement));
    langs
}

#[function_component(Skills)]
pub fn skills() -> Html {
    html!(
        <div class="container">
            <nav class="level">
                { for ["Languages", "Methods", "Technologies"].into_iter().map(|item| {
                    html!(<div class="level-item">
                        <h1 class="title is-1">{ item }</h1>
                    </div>)
                })}
            </nav>
            <div class="box has-background-primary-light">
                { for sorted_skills().into_iter().map(|skill| {
                    html!(<SkillComponent ..skill.clone() />)
                })}
            </div>
        </div>
    )
}

#[function_component(SkillComponent)]
pub fn skill(skill: &Skill) -> Html {
    html!(
        <div class="columns is-centered is-vcentered is-6">
            <div class="column is-2 is-full-height">
                <h1 class="title is-4">{ skill.title }</h1>
            </div>
            <div class="column is-5">
                <span class="level is-mobile">
                    <span class="icon is-large has-text-danger mr-1">
                        <i class="mdi mdi-36px mdi-heart"></i>
                    </span>
                    <progress class="progress is-medium is-danger" value={format!("{}", skill.excitement)} max="100">{ format!("{}%", skill.excitement) }</progress>
                </span>
            </div>
            <div class="column is-5">
                <span class="level is-mobile">
                    <span class="icon is-large has-text-success mr-1">
                        <i class="mdi mdi-36px mdi-lightbulb-on"></i>
                    </span>
                    <progress class="progress is-medium is-success" value={format!("{}", skill.skill)} max="100">{ format!("{}%", skill.skill) }</progress>
                </span>
            </div>
        </div>
    )
}
