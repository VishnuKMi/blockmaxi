use gloo_net::http::Request;
use leptos::*;
use shared::Project;

#[component]
pub fn Projects() -> impl IntoView {
    let projects = create_resource(
        || (),
        |_| async {
            Request::get("http://localhost:3000/api/projects")
                .send()
                .await
                .ok()?
                .json::<Vec<Project>>()
                .await
                .ok()
        },
    );

    view! {
        <section id="projects" class="min-h-screen py-32 px-6">
            <div class="container mx-auto max-w-7xl">
                <div class="text-center mb-20 animate-on-scroll">
                    <h2 class="text-5xl md:text-6xl font-bold gradient-text mb-4">
                        "Featured Projects"
                    </h2>
                    <p class="text-xl text-slate-400 max-w-2xl mx-auto">
                        "Explore my latest blockchain and Web3 innovations"
                    </p>
                </div>

                <Suspense fallback=move || view! {
                    <div class="grid md:grid-cols-2 gap-8">
                        {(0..4).map(|_| view! {
                            <div class="h-96 rounded-2xl skeleton"></div>
                        }).collect::<Vec<_>>()}
                    </div>
                }>
                    {move || {
                        projects.get().and_then(|data| {
                            data.map(|projects| view! {
                                <div class="grid md:grid-cols-2 gap-8">
                                    {projects.into_iter().enumerate().map(|(i, project)| view! {
                                        <div class="animate-on-scroll" style=format!("animation-delay: {}s", i as f32 * 0.1)>
                                            <ProjectCard project=project/>
                                        </div>
                                    }).collect::<Vec<_>>()}
                                </div>
                            })
                        })
                    }}
                </Suspense>
            </div>
        </section>
    }
}

#[component]
fn ProjectCard(project: Project) -> impl IntoView {
    view! {
        <div class="group glass-effect rounded-2xl overflow-hidden hover-lift card-reveal h-full">
            // Project image/icon
            <div class="relative h-56 bg-gradient-to-br from-cyan-500/20 to-blue-500/20 flex items-center justify-center text-7xl overflow-hidden">
                <div class="absolute inset-0 bg-gradient-to-br from-cyan-500/10 to-blue-500/10 group-hover:scale-110 transition-transform duration-500"></div>
                <span class="relative z-10 group-hover:scale-110 transition-transform duration-300">"🚀"</span>
            </div>

            <div class="p-8 space-y-6">
                <div>
                    <h3 class="text-2xl font-bold mb-3 group-hover:text-cyan-400 transition-colors">
                        {project.title}
                    </h3>
                    <p class="text-slate-300 leading-relaxed">
                        {project.description}
                    </p>
                </div>

                // Tech stack
                <div class="flex flex-wrap gap-2">
                    {project.tech_stack.into_iter().map(|tech| view! {
                        <span class="px-3 py-1.5 bg-cyan-500/10 text-cyan-400 rounded-lg text-sm font-mono border border-cyan-500/20 hover:bg-cyan-500/20 transition-all">
                            {tech}
                        </span>
                    }).collect::<Vec<_>>()}
                </div>

                // Links
                <div class="flex gap-4 pt-4 border-t border-slate-700">
                    {project.github_url.map(|url| view! {
                        <a href=url target="_blank" class="flex items-center gap-2 text-cyan-400 hover:text-cyan-300 transition-colors group/link">
                            <svg class="w-5 h-5 group-hover/link:scale-110 transition-transform" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                            </svg>
                            "GitHub"
                        </a>
                    })}
                    {project.live_url.map(|url| view! {
                        <a href=url target="_blank" class="flex items-center gap-2 text-blue-400 hover:text-blue-300 transition-colors group/link">
                            <svg class="w-5 h-5 group-hover/link:scale-110 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"/>
                            </svg>
                            "Live Demo"
                        </a>
                    })}
                </div>
            </div>
        </div>
    }
}
