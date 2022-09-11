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

    let menu_active = use_state(|| false);
    let toggle_menu = {
        let menu_active = menu_active.clone();
        Callback::from(move |_| menu_active.set(!*menu_active))
    };

    html!(
      <>
      <nav class="navbar is-dark" role="navigation" aria-label="main navigation">
        <div class="navbar-brand">
          <a class="navbar-item" href="http://butzist.github.io/">
            <img src="assets/szalkowski.png" />
          </a>

          <a role="button" class={classes!("navbar-burger", menu_active.then(|| "is-active"))} aria-label="menu" aria-expanded="false" data-target="navbar" onclick={toggle_menu.clone()}>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
          </a>
        </div>

        <div id="navbar" class={classes!("navbar-menu", menu_active.then(|| "is-active"))}>
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

          <div class="navbar-end">
            <a class="navbar-item" href="http://github.com/butzist/butzist.github.io/">
              <img src="assets/GitHub-Mark-Light-32px.png" alt="View page source"/>
            </a>
          </div>
        </div>
      </nav>

      <div class="px-6 pt-6">
      { (*content).clone() }
      </div>
    </>
    )
}

fn main() {
    yew::start_app::<App>();
}
