use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::project_card::ProjectCard;
use crate::data::projects::get_projects;

#[component]
pub fn Home() -> impl IntoView {
    let projects = get_projects();

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
            </section>

            // Skills
            <section class="skills-preview">
                <h2>"Core Competencies"</h2>
                <div class="skills-grid">
                    <div class="skill-card">
                        <h3>"Mechanical Design"</h3>
                        <p>"SolidWorks, GD&T, DFM, mechanism design, structural systems."</p>
                    </div>
                    <div class="skill-card">
                        <h3>"Simulation & Analysis"</h3>
                        <p>"FEA, CFD fundamentals, turbulence research, experimental validation."</p>
                    </div>
                    <div class="skill-card">
                        <h3>"Computational Engineering"</h3>
                        <p>"Rust, Python, data analysis, numerical methods, WASM systems."</p>
                    </div>
                    <div class="skill-card">
                        <h3>"Manufacturing & Testing"</h3>
                        <p>"Fabrication workflows, rapid prototyping, system validation."</p>
                    </div>
                </div>
            </section>

            // Featured Projects
            <section class="featured-projects">
                <div class="projects-header">
                    <p class="projects-eyebrow">"// featured work"</p>
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
                <div class="projects-cta">
                    <A href="/projects">
                        <span class="btn-secondary">"View All Projects →"</span>
                    </A>
                </div>
            </section>
        </main>
    }
}