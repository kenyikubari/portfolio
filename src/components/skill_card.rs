use leptos::prelude::*;
use crate::data::skills::Skill;

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