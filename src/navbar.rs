use crate::projects;
use yew::prelude::*;

#[derive(Clone, Copy)]
pub enum ContentPage {
    About,
    Languages,
    Projects,
}

#[derive(Properties, PartialEq)]
pub struct NavProps {
    pub set_content: Callback<ContentPage>,
}

#[function_component(Nav)]
pub fn nav(props: &NavProps) -> Html {
    let goto = |page: ContentPage| {
        let setter = props.set_content.clone();
        Callback::from(move |_| setter.emit(page))
    };

    let menu_active = use_state(|| false);
    let toggle_menu = {
        let menu_active = menu_active.clone();
        Callback::from(move |_| menu_active.set(!*menu_active))
    };

    html!(
      <nav class="navbar is-dark" role="navigation" aria-label="main navigation">
        <div class="navbar-brand">
          <a class="navbar-item" href="http://butzist.github.io/">
            <img src="assets/szalkowski.png" />
          </a>

          <a role="button" class={classes!("navbar-burger", menu_active.then(|| "is-active"))} aria-label="menu" aria-expanded="false" onclick={toggle_menu.clone()}>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
          </a>
        </div>

        <div class={classes!("navbar-menu", menu_active.then(|| "is-active"))}>
          <div class="navbar-start">
            <div class="navbar-item has-dropdown is-hoverable">
              <a class="navbar-link" onclick={goto(ContentPage::Projects)}>
                { "Projects" }
              </a>

              <div class="navbar-dropdown">
                {
                  for projects::PROJECTS.with(|f| f.clone()).into_iter().map(|project| { html!(
                    <a class="navbar-item" href={ project.link }>
                    { project.title }
                  </a>
                  ) })
                }
              </div>
            </div>

            <a class="navbar-item" onclick={goto(ContentPage::Languages)}>
              { "Languages" }
            </a>

            <a class="navbar-item" onclick={goto(ContentPage::About)}>
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
    )
}
