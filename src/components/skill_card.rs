use leptos::prelude::*;
use crate::data::skills::Skill;

<<<<<<< HEAD
=======
fn get_initials(title: &str) -> String {
    title
        .split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(2)
        .collect::<String>()
        .to_uppercase()
}

>>>>>>> b082272705f38c8d61ae245ad3e1a4006ea1226c
#[component]
pub fn SkillCard(skill: Skill) -> impl IntoView {
    view! {
        <article class="skill-card">
            <div class="skill-body">
                <h3 class="skill-title">{skill.title}</h3>
                <p class="skill-desc">{skill.description}</p>
                <ul class="skill-tags">
                    {skill.tags.into_iter().map(|tag| view! {
                        <li class="skill-tag">{tag}</li>
                    }).collect_view()}
                </ul>
            </div>
        </article>
    }
}