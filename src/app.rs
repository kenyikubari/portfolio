use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
use crate::components::navigation_bar::Navbar;
use crate::pages::home::Home;
use crate::pages::about::About;
use crate::pages::projects::Projects;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Navbar />
            <main>
                <Routes fallback=|| view! { <Home /> }>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/about") view=About />
                    <Route path=path!("/projects") view=Projects />
                </Routes>
            </main>
        </Router>
    }
}