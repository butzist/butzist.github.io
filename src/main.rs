use yew::prelude::*;

mod about;
use about::About;
mod projects;
use projects::Projects;

#[function_component(App)]
fn app() -> Html {
    let content = use_state(|| html!(<Projects />));
    fn goto<T: Component<Properties = ()>>(content: UseStateHandle<Html>) -> Callback<MouseEvent> {
        Callback::from(move |_| content.set(html!(<T />)))
    }

    html!(
      <>
      <nav class="navbar" role="navigation" aria-label="main navigation">
        <div class="navbar-brand">
          <a class="navbar-item" href="http://butzist.github.io/">
            <img src="assets/szalkowski.png" />
          </a>

          <a role="button" class="navbar-burger" aria-label="menu" aria-expanded="false" data-target="navbar">
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
          </a>
        </div>

        <div id="navbar" class="navbar-menu">
          <div class="navbar-start">
            <div class="navbar-item has-dropdown is-hoverable">
              <a class="navbar-link" onclick={goto::<Projects>(content.clone())}>
                { "Projects" }
              </a>

              <div class="navbar-dropdown">
                {
                  for projects::PROJECTS.iter().map(|project| { html!(
                    <a class="navbar-item" href={ project.link }>
                    { project.title }
                  </a>
                  ) })
                }
              </div>
            </div>

            <a class="navbar-item" onclick={goto::<About>(content.clone())}>
            { "About" }
            </a>
          </div>
        </div>
      </nav>

      <div class="px-6">
      { (*content).clone() }
      </div>
    </>
    )
}

fn main() {
    yew::start_app::<App>();
}
