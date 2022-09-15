use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Project {
    pub title: &'static str,
    pub inactive: bool,
    pub description: Html,
    pub language: &'static str,
    pub image: Option<&'static str>,
    pub link: &'static str,
}

thread_local! {
    pub static PROJECTS: Vec<Project> = vec![
        Project {
            title: "Activity Launcher",
            inactive: false,
            description:
                html!({
                    "Android App that launches hidden activities and creates home screen shortcuts for installed apps."
                }),
            language: "Java",
            image: Some("assets/activity_launcher.png"),
            link: "https://play.google.com/store/apps/details?id=de.szalkowski.activitylauncher",
        },
        Project {
            title: "Te*ris",
            inactive: false,
            description:
                html!({ "Cross-platform Te*ris clone written with Rust and bevy" }),
            language: "Rust",
            image: Some("assets/tetris.png"),
            link: "https://butzist.github.io/tetris",
        },
        Project {
            title: "DevOpsDemo",
            inactive: false,
            description: html!({
                    "DevOps demo project featuring Terraform, Kubernetes, Azure Pipelines, Spinnaker, and micro-service templates for several languages."
                }),
            language: "Terraform, Go, Rust, Python, ...",
            image: Some("assets/devopsdemo.png"),
            link: "https://github.com/DevOpsDemoTF",
        },
        Project {
            title: "OpenGL experiments",
            inactive: true,
            description: html!({ "Experiments with OpenGL in Rust" }) ,
            language: "Rust + GLSL",
            image: Some("https://bulma.io/images/placeholders/128x128.png"),
            link: "https://github.com/butzist/piston-cube",
        },
        Project {
            title: "BatterySaver",
            inactive: true,
            description:
                html!({
                    "Android app that saves power by toggling network interfaces periodically and based on device power state."
                })
            ,
            language: "Java",
            image: Some("assets/battery_saver.png"),
            link: "https://github.com/butzist/BatterySaver",
        },
        Project {
            title: "ProGraphMSA",
            inactive: true,
            description:
                html!({
                    "Graph-based Multiple Sequence Alignment for amino-acid sequences with intrinsic disorder and tandem repeats (PhD thesis)"
                })
            ,
            language: "C++",
            image: Some("https://bulma.io/images/placeholders/128x128.png"),
            link: "https://github.com/butzist/ProGraphMSA",
        },
    ];
}

#[function_component(Projects)]
pub fn projects() -> Html {
    html!(
        <div class="container">
            <h1 class="title is-1 has-text-centered">{ "Private projects" }</h1>
            <div class="columns is-multiline is-6">
                { for PROJECTS.with(|f| f.clone()).into_iter().map(|project| {
                    html!(<ProjectComponent ..project.clone() />)
                })}
            </div>
        </div>
    )
}

#[function_component(ProjectComponent)]
pub fn project(project: &Project) -> Html {
    html!(
        <div class="column is-half">
            <div class="card is-full-height has-background-primary-light">
                <div class="card-content">
                    <div class="media">
                        <div class="media-left">
                            <figure class="image is-64x64">
                                <img src={project.image} alt="Project image" />
                            </figure>
                        </div>
                        <div class="media-content">
                            <p class="title is-4">
                                { project.title }
                                if project.inactive { <sup><span class="tag ml-2 is-warning">{ "inactive" }</span></sup> }
                            </p>
                            <p class="subtitle is-6">{ project.language }</p>
                            <p>{ project.description.clone() }</p>
                        </div>
                    </div>
                </div>
                <div class="card-footer">
                <a class="card-footer-item button is-link is-outlined is-rounded is-has-text-weight-bold" href={ project.link }>{ "Website " }</a>
                </div>
            </div>
        </div>
    )
}
