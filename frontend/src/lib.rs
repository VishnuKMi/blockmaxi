use leptos::*;

mod components;
use components::{Contact, Experience, Hero, Navbar, Projects, Skills};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gradient-to-br from-slate-950 via-slate-900 to-slate-950 text-white">
            <Navbar/>
            <Hero/>
            <Projects/>
            <Skills/>
            <Experience/>
            <Contact/>
            <Footer/>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-slate-800 py-8 text-center text-slate-400">
            <p>"Built with Rust 🦀 | © 2025"</p>
        </footer>
    }
}

#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}
