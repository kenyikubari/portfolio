use leptos::prelude::*;
use crate::data::projects::Project;

fn get_initials(title: &str) -> String {
    title
        .split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    let initials = get_initials(&project.title);
    let hue: u16 = (project.title.bytes().fold(0u32, |acc, b| acc + b as u32) % 360) as u16;
    let thumb_style = format!(
        "background: linear-gradient(135deg, hsl({hue},40%,18%) 0%, hsl({},40%,12%) 100%);",
        (hue + 40) % 360
    );

    view! {
        <article class="project-card">
            <div class="project-thumb" style=thumb_style>
                <span class="project-initials">{initials}</span>
                <div class="project-thumb-grid"></div>
            </div>

            <div class="project-body">
                <h3 class="project-title">{project.title}</h3>
                <p class="project-desc">{project.description}</p>

                <ul class="project-tags">
                    {project.tags.into_iter().map(|tag| view! {
                        <li class="project-tag">{tag}</li>
                    }).collect_view()}
                </ul>

                <div class="project-links">
                    {project.link.map(|url| view! {
                        <a href=url target="_blank" class="btn btn-primary">"↗ Live"</a>
                    })}
                    {project.repo.map(|url| view! {
                        <a href=url target="_blank" class="btn btn-ghost">"{ } Repo"</a>
                    })}
                </div>
            </div>
        </article>
    }
}