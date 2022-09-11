use yew::prelude::*;

#[function_component(About)]
pub fn projects() -> Html {
    html!(
        <div class="columns is-centered is-multiline is-6">
            <div class="column is-two-thirds">
                <div class="card">
                    <div class="card-content">
                        <h1 class="title">{ "Current job" }</h1>
                        <div class="media">
                            <div class="media-left">
                                <figure class="image is-128x128">
                                    <img src="assets/datahow.jpeg" alt="DataHow" />
                                </figure>
                            </div>
                            <div class="media-content">
                                <p class="title is-4">
                                    { "DataHow AG" }
                                </p>
                                <p class="subtitle is-6">{ "Lead Software Engineer" }</p>
                                <a class="subtitle is-6" href="https://www.datahow.ch/">{ "https://www.datahow.ch/" }</a>
                            </div>
                        </div>

                        <div class="content">
                            {
                                "DataHow is a ETH Spin-off that provides software for bioprocess data analysis.
                                My team is working on a modular software platform for data analysis through a web client.
                                The software is built with micro-services and micro-frontends on top of Kubernetes allowing
                                a deployment on any cloud platform or on premise."
                            }
                        </div>
                    </div>
                </div>
            </div>

            <div class="column is-two-thirds">
                <div class="card">
                    <div class="card-content">
                        <h1 class="title">{ "Links" }</h1>
                        <div>
                            <ul>
                                <li><a href="https://ch.linkedin.com/in/adam-szalkowski-6067b2106">{ "LinkedIn Profile" }</a></li>
                                <li><a href="https://github.com/butzist">{ "GitHub Profile" }</a></li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}
