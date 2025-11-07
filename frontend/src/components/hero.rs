use leptos::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="relative min-h-screen flex items-center justify-center px-6 pt-20 overflow-hidden">
            // Animated background elements
            <div class="absolute inset-0 overflow-hidden pointer-events-none">
                <div class="absolute top-1/4 left-1/4 w-96 h-96 bg-cyan-500/10 rounded-full blur-3xl animate-float"></div>
                <div class="absolute bottom-1/4 right-1/4 w-96 h-96 bg-blue-500/10 rounded-full blur-3xl animate-float" style="animation-delay: 2s;"></div>
            </div>

            <div class="hero-content text-center space-y-8 max-w-5xl mx-auto relative z-10">
                // Animated avatar
                <div class="inline-block mb-6 animate-fade-in-up">
                    <div class="w-32 h-32 rounded-full bg-gradient-to-r from-cyan-500 to-blue-500 p-1 animate-glow">
                        <div class="w-full h-full rounded-full bg-slate-900 flex items-center justify-center text-6xl">
                            "🔗"
                        </div>
                    </div>
                </div>

                // Main heading with stagger animation
                <div class="space-y-4">
                    <h1 class="text-6xl md:text-8xl font-bold gradient-text animate-fade-in-up stagger-1">
                        "Blockchain Engineer"
                    </h1>
                    <p class="text-xl md:text-2xl text-slate-300 max-w-3xl mx-auto leading-relaxed animate-fade-in-up stagger-2" style="font-family: var(--font-secondary);">
                        "Building the decentralized future with "
                        <span class="text-cyan-400 font-semibold">"smart contracts"</span>
                        ", "
                        <span class="text-blue-400 font-semibold">"DeFi protocols"</span>
                        ", and "
                        <span class="text-purple-400 font-semibold">"Web3 innovation"</span>
                    </p>
                </div>

                // Tech stack badges
                <div class="flex flex-wrap gap-3 justify-center animate-fade-in-up stagger-3">
                    <span class="px-4 py-2 bg-slate-800/50 border border-slate-700 rounded-full text-sm font-mono hover:border-cyan-500 transition-all hover:scale-105">
                        "Solidity"
                    </span>
                    <span class="px-4 py-2 bg-slate-800/50 border border-slate-700 rounded-full text-sm font-mono hover:border-cyan-500 transition-all hover:scale-105">
                        "Rust"
                    </span>
                    <span class="px-4 py-2 bg-slate-800/50 border border-slate-700 rounded-full text-sm font-mono hover:border-cyan-500 transition-all hover:scale-105">
                        "Web3.js"
                    </span>
                    <span class="px-4 py-2 bg-slate-800/50 border border-slate-700 rounded-full text-sm font-mono hover:border-cyan-500 transition-all hover:scale-105">
                        "Ethereum"
                    </span>
                </div>

                // CTA buttons
                <div class="flex gap-4 justify-center pt-6 animate-fade-in-up stagger-4">
                    <a href="#projects" class="btn-primary px-8 py-4 bg-gradient-to-r from-cyan-500 to-blue-500 rounded-xl hover:shadow-2xl hover:shadow-cyan-500/50 transition-all font-semibold text-lg relative z-10">
                        "View Projects"
                    </a>
                    <a href="#contact" class="px-8 py-4 border-2 border-cyan-500 rounded-xl hover:bg-cyan-500/10 transition-all font-semibold text-lg hover:scale-105">
                        "Get in Touch"
                    </a>
                </div>

                // Scroll indicator
                <div class="absolute bottom-10 left-1/2 transform -translate-x-1/2 animate-bounce">
                    <svg class="w-6 h-6 text-cyan-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3"/>
                    </svg>
                </div>
            </div>
        </section>
    }
}
