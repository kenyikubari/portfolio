use leptos::prelude::*;
use crate::data::curriculum_vitae::get_cv;

fn date_range(start: &str, end: &str) -> String {
    if end.is_empty() {
        format!("{} — Present", start)
    } else {
        format!("{} — {}", start, end)
    }
}

fn date_or_range(date: &Option<String>, start: &Option<String>, end: &Option<String>) -> String {
    if let Some(d) = date {
        return d.clone();
    }
    let s = start.clone().unwrap_or_default();
    let e = end.clone().unwrap_or_default();
    date_range(&s, &e)
}

#[component]
pub fn Cv() -> impl IntoView {
    let cv = get_cv();

    view! {
        <main class="cv-page">
            <div class="cv-body container">

            // ── Header ───────────────────────────────────────────────
            <header class="cv-header">
                <p class="cv-eyebrow">"// curriculum vitae"</p>
                <h1 class="cv-name">"Kenyi Kubari"</h1>
                <p class="cv-subtitle">"Mechanical Engineer · Researcher · Software Developer"</p>
                <a href="assets/docs/kenyikubari_resume_MB.pdf" target="_blank" class="cv-download">
                    "↓ Download PDF"
                </a>
            </header>

            <div class="cv-body">

                // ── Education ────────────────────────────────────────
                <section class="cv-section">
                    <h2 class="cv-section-title">"Education"</h2>
                    {cv.education.entry.into_iter().map(|e| view! {
                        <div class="cv-entry">
                            <div class="cv-entry-header">
                                <div>
                                    <p class="cv-entry-title">{e.institution}</p>
                                    <p class="cv-entry-sub">{e.degree}</p>
                                    {e.minor.map(|m| view! {
                                        <p class="cv-entry-minor">"Minor: " {m}</p>
                                    })}
                                </div>
                                <div class="cv-entry-meta">
                                    <span class="cv-location">{e.location}</span>
                                    <span class="cv-date">{date_range(&e.start, &e.end)}</span>
                                </div>
                            </div>
                            {e.awards.map(|awards: Vec<String>| view! {
                                <div class="cv-awards">
                                    <p class="cv-label">"Awards"</p>
                                    <ul class="cv-tags">
                                        {awards.into_iter().map(|a| view! {
                                            <li class="cv-tag">{a}</li>
                                        }).collect_view()}
                                    </ul>
                                </div>
                            })}
                            {e.courses.map(|courses: Vec<String>| view! {
                                <div class="cv-courses">
                                    <p class="cv-label">"Relevant Coursework"</p>
                                    <ul class="cv-tags">
                                        {courses.into_iter().map(|c| view! {
                                            <li class="cv-tag">{c}</li>
                                        }).collect_view()}
                                    </ul>
                                </div>
                            })}
                        </div>
                    }).collect_view()}
                </section>

                // ── Work Experience ───────────────────────────────────
                <section class="cv-section">
                    <h2 class="cv-section-title">"Work Experience"</h2>
                    {cv.work_experience.into_iter().map(|w| view! {
                        <div class="cv-entry">
                            <div class="cv-entry-header">
                                <div>
                                    <p class="cv-entry-title">{w.title}</p>
                                    <p class="cv-entry-sub">{w.organization}</p>
                                </div>
                                <div class="cv-entry-meta">
                                    <span class="cv-location">{w.location}</span>
                                    <span class="cv-date">{date_range(&w.start, &w.end)}</span>
                                </div>
                            </div>
                            <ul class="cv-bullets">
                                {w.bullets.into_iter().map(|b| view! {
                                    <li>{b}</li>
                                }).collect_view()}
                            </ul>
                        </div>
                    }).collect_view()}
                </section>

                // ── Volunteer ─────────────────────────────────────────
                <section class="cv-section">
                    <h2 class="cv-section-title">"Volunteer Experience"</h2>
                    {cv.volunteer_experience.into_iter().map(|v| view! {
                        <div class="cv-entry cv-entry-compact">
                            <div class="cv-entry-header">
                                <div>
                                    <p class="cv-entry-title">{v.title}</p>
                                    <p class="cv-entry-sub">{v.organization}</p>
                                </div>
                                <div class="cv-entry-meta">
                                    {v.location.map(|l| view! {
                                        <span class="cv-location">{l}</span>
                                    })}
                                    <span class="cv-date">
                                        {date_or_range(&v.date, &v.start, &v.end)}
                                    </span>
                                </div>
                            </div>
                        </div>
                    }).collect_view()}
                </section>

                // ── Athletics ─────────────────────────────────────────
                <section class="cv-section">
                    <h2 class="cv-section-title">"Athletics"</h2>
                    {cv.athletics.into_iter().map(|a| view! {
                        <div class="cv-entry">
                            <div class="cv-entry-header">
                                <div>
                                    <p class="cv-entry-title">{a.team}</p>
                                    <p class="cv-entry-sub">{a.role}</p>
                                </div>
                                <div class="cv-entry-meta">
                                    <span class="cv-location">{a.location}</span>
                                    <span class="cv-date">{date_range(&a.start, &a.end)}</span>
                                </div>
                            </div>
                            <ul class="cv-bullets">
                                {a.achievements.into_iter().map(|ach| view! {
                                    <li>{ach}</li>
                                }).collect_view()}
                            </ul>
                        </div>
                    }).collect_view()}
                </section>

                // ── Extracurriculars ──────────────────────────────────
                <section class="cv-section">
                    <h2 class="cv-section-title">"Extracurriculars"</h2>
                    {cv.extracurricular.into_iter().map(|e| view! {
                        <div class="cv-entry">
                            <div class="cv-entry-header">
                                <div>
                                    <p class="cv-entry-title">{e.title}</p>
                                    <p class="cv-entry-sub">{e.organization}</p>
                                </div>
                                <div class="cv-entry-meta">
                                    <span class="cv-location">{e.location}</span>
                                    <span class="cv-date">{date_range(&e.start, &e.end)}</span>
                                </div>
                            </div>
                            <ul class="cv-bullets">
                                {e.achievements.into_iter().map(|a| view! {
                                    <li>{a}</li>
                                }).collect_view()}
                            </ul>
                        </div>
                    }).collect_view()}
                </section>

                // ── Bottom row: Memberships, Languages, Interests ─────
                <div class="cv-bottom-grid">

                    <section class="cv-section">
                        <h2 class="cv-section-title">"Memberships"</h2>
                        {cv.membership.into_iter().map(|m| view! {
                            <div class="cv-entry cv-entry-compact">
                                <p class="cv-entry-title">
                                    {m.abbreviation.unwrap_or(m.organization)}
                                </p>
                                <p class="cv-entry-sub">{m.role} " · since " {m.start}</p>
                            </div>
                        }).collect_view()}
                    </section>

                    <section class="cv-section">
                        <h2 class="cv-section-title">"Languages"</h2>
                        {cv.language.into_iter().map(|l| view! {
                            <div class="cv-entry cv-entry-compact">
                                <p class="cv-entry-title">{l.name}</p>
                                <p class="cv-entry-sub">{l.level}</p>
                            </div>
                        }).collect_view()}
                    </section>

                    <section class="cv-section">
                        <h2 class="cv-section-title">"Interests"</h2>
                        <ul class="cv-tags">
                            {cv.interest.into_iter().map(|i| view! {
                                <li class="cv-tag">{i.name}</li>
                            }).collect_view()}
                        </ul>
                    </section>

                </div>
            </div>
            </div>
        </main>
    }
}
