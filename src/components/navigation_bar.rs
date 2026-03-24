use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="navbar">
            <div class="nav-container">

                <div class="nav-logo">
                    <A href="/">"Kenyi Kubari"</A>
                </div>

                <div class="nav-links">
                    <A href="/">"Home"</A>
                    <A href="/about">"About"</A>
                    <A href="/cv">"CV"</A>
                    <A href="/projects">"Projects"</A>
                </div>

            </div>
        </nav>
    }
}
