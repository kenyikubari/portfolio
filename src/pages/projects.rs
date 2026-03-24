use leptos::prelude::*;
use crate::components::project_card::ProjectCard;
use crate::data::projects::get_projects;

#[component]
pub fn Projects() -> impl IntoView {
    let projects = get_projects();

    view! {
        <section class="projects-section">
            <header class="projects-header">
                <p class="projects-eyebrow">"// selected work"</p>
                <h2 class="projects-heading">
                    "Engineering"
                    <span class="heading-accent">" Projects"</span>
                </h2>
                <div class="projects-rule"></div>
            </header>

            <div class="projects-grid">
                {projects.into_iter().map(|p| view! {
                    <ProjectCard project=p />
                }).collect_view()}
            </div>
        </section>
    }
}