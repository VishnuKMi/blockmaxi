use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="fixed top-0 w-full z-50 transition-all duration-300">
            <div class="container mx-auto px-6 py-5">
                <div class="flex justify-between items-center">
                    // Logo with hover effect
                    <a href="#" class="group">
                        <h1 class="text-2xl font-bold gradient-text group-hover:scale-105 transition-transform">
                            "⟨/⟩ Portfolio"
                        </h1>
                    </a>

                    // Navigation links
                    <div class="hidden md:flex gap-8 items-center">
                        <a href="#projects" class="relative group text-slate-300 hover:text-white transition-colors">
                            "Projects"
                            <span class="absolute bottom-0 left-0 w-0 h-0.5 bg-gradient-to-r from-cyan-500 to-blue-500 group-hover:w-full transition-all duration-300"></span>
                        </a>
                        <a href="#skills" class="relative group text-slate-300 hover:text-white transition-colors">
                            "Skills"
                            <span class="absolute bottom-0 left-0 w-0 h-0.5 bg-gradient-to-r from-cyan-500 to-blue-500 group-hover:w-full transition-all duration-300"></span>
                        </a>
                        <a href="#experience" class="relative group text-slate-300 hover:text-white transition-colors">
                            "Experience"
                            <span class="absolute bottom-0 left-0 w-0 h-0.5 bg-gradient-to-r from-cyan-500 to-blue-500 group-hover:w-full transition-all duration-300"></span>
                        </a>
                        <a href="#contact" class="px-6 py-2 bg-gradient-to-r from-cyan-500 to-blue-500 rounded-lg hover:shadow-lg hover:shadow-cyan-500/50 transition-all hover:scale-105 font-semibold">
                            "Contact"
                        </a>
                    </div>

                    // Mobile menu button
                    <button class="md:hidden text-white">
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
                        </svg>
                    </button>
                </div>
            </div>
        </nav>
    }
}
