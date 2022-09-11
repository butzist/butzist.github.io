use lazy_static::lazy_static;
use yew::prelude::*;

pub struct Project {
    pub title: &'static str,
    pub description: &'static (dyn Fn() -> Html + Sync),
    pub language: &'static str,
    pub image: Option<&'static str>,
    pub link: &'static str,
}

lazy_static! {
    pub static ref PROJECTS: Vec<Project> = vec![
        Project {
            title: "Activity Launcher",
            description: &|| {
                html!({
                    "Android App that launches hidden activities and creates home screen shortcuts for installed apps."
                })
            },
            language: "Java",
            image: Some("assets/activity_launcher.png"),
            link: "https://play.google.com/store/apps/details?id=de.szalkowski.activitylauncher",
        },
        Project {
            title: "Te*ris",
            description: &|| { html!({ "Te*ris clone written with Rust and bevy" }) },
            language: "Rust",
            image: Some("assets/tetris.png"),
            link: "https://github.com/butzist/tetris",
        },
        Project {
            title: "DevOpsDemo",
            description: &|| {
                html!({
                    "DevOps demo project featuring Terraform, Kubernetes, Azure Pipelines, Spinnaker, and micro-service templates for several languages."
                })
            },
            language: "Terraform, Go, Rust, Python, ...",
            image: Some("assets/devopsdemo.png"),
            link: "https://github.com/DevOpsDemoTF",
        },
        Project {
            title: "OpenGL experiments",
            description: &|| { html!({ "Experiments with OpenGL in Rust" }) },
            language: "Rust + GLSL",
            image: Some("https://bulma.io/images/placeholders/128x128.png"),
            link: "https://github.com/butzist/piston-cube",
        },
        Project {
            title: "BatterySaver",
            description: &|| {
                html!({
                    "Android app that saves power by toggling network interfaces periodically and based on device power state."
                })
            },
            language: "Java",
            image: Some("assets/battery_saver.png"),
            link: "https://github.com/butzist/BatterySaver",
        },
        Project {
            title: "ProGraphMSA",
            description: &|| {
                html!({
                    "Graph-based Multiple Sequence Alignment for amino-acid sequences with intrinsic disorder and tandem repeats (PhD thesis)"
                })
            },
            language: "C++",
            image: Some("https://bulma.io/images/placeholders/128x128.png"),
            link: "https://github.com/butzist/ProGraphMSA",
        },
    ];
}

#[function_component(Projects)]
pub fn projects() -> Html {
    html!(
      <>
      <h1 class="title">{ "Private projects" }</h1>
      <div class="columns is-multiline is-6">
      { for PROJECTS.iter().map(|project| {
        html!(
          <div class="column" style="min-width: 500px">
            <div class="card is-full-height">
              <div class="card-content">
                <div class="media">
                  <div class="media-left">
                    <figure class="image is-128x128">
                      <img src={project.image} alt="Project image" />
                    </figure>
                  </div>
                  <div class="media-content">
                    <p class="title is-4">{ project.title }</p>
                    <p class="subtitle is-6">{ project.language }</p>
                    <a class="subtitle is-6" href={ project.link }>{ project.link }</a>
                  </div>
                </div>

                <div class="content">
                  { (project.description)() }
                </div>
              </div>
            </div>
          </div>
      )})}
      </div>
      </>
    )
}
