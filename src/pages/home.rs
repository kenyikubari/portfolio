use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::project_card::ProjectCard;
use crate::components::skill_card::SkillCard;
use crate::data::projects::get_projects;
use crate::data::skills::get_skills;
use crate::data::contact::get_contact;

#[component]
pub fn Home() -> impl IntoView {
    let projects = get_projects();
    let skills = get_skills();
    let contact = get_contact();

    view! {
        
        <main class="home">
            // Hero
            <section class="hero">
                <p class="hero-eyebrow">">>> [mechanical engineer, researcher, software developer];"</p>
                <h1>"Kenyi Kubari"</h1>
                <h2>"Systems Designer · Fluid Mechanics · Computational Engineering"</h2>
                <p class="hero-description">
                    "I design robust mechanical systems at the intersection of fluid mechanics, \
                    structural analysis, and computational engineering. From experimental turbulence \
                    research to high-performance SAE powertrain systems, I build with precision and purpose."
                </p>
                <div class="hero-buttons">
                    <A href="/projects">
                        <span class="btn-primary">"View Projects"</span>
                    </A>
                    <A href="/about">
                        <span class="btn-secondary">"About Me"</span>
                    </A>
                </div>
                <div class="hero-socials">
                    {contact.github.map(|url| view! {
                        <a href=url target="_blank" class="social-link">
                            <i class="fab fa-github"></i>
                        </a>
                    })}
                    {contact.linkedin.map(|url| view! {
                        <a href=url target="_blank" class="social-link">
                            <i class="fab fa-linkedin"></i>
                        </a>
                    })}
                    {contact.research_gate.map(|url| view! {
                        <a href=url target="_blank" class="social-link">
                            <i class="fas fa-flask"></i>
                        </a>
                    })}
                    {contact.youtube.map(|url| view! {
                        <a href=url target="_blank" class="social-link">
                            <i class="fab fa-youtube"></i>
                        </a>
                    })}
                    {contact.instagram.map(|url| view! {
                        <a href=url target="_blank" class="social-link">
                            <i class="fab fa-instagram"></i>
                        </a>
                    })}
                    {contact.tiktok.map(|url| view! {
                        <a href=url target="_blank" class="social-link">
                            <i class="fab fa-tiktok"></i>
                        </a>
                    })}
                </div>
            </section>

            // Skills
            <section class="skills-preview">
                <div class="projects-header">
                    <p class="projects-eyebrow">"// skills we learned along the way;"</p>
                    <h2 class="projects-heading">
                        "Core"
                        <span class="heading-accent">" Competencies"</span>
                    </h2>
                    <div class="projects-rule"></div>
                </div>
                <div class="skills-grid">
                    {skills.into_iter().map(|s| view! {
                        <SkillCard skill=s />
                    }).collect_view()}
                </div>
            </section>

            // Featured Projects
            <section class="featured-projects">
                <div class="projects-header">
                    <p class="projects-eyebrow">"// featured work;"</p>
                    <h2 class="projects-heading">
                        "Selected"
                        <span class="heading-accent">" Projects"</span>
                    </h2>
                    <div class="projects-rule"></div>
                </div>
                <div class="projects-grid">
                    {projects.into_iter().take(2).map(|p| view! {
                        <ProjectCard project=p />
                    }).collect_view()}
                </div>
                <br></br>
                <div class="projects-cta">
                    <A href="/projects">
                        <span class="btn-primary">"View All Projects →"</span>
                    </A>
                </div>
            </section>
        </main>
    }
}