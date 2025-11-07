use gloo_net::http::Request;
use leptos::*;
use shared::Skill;

#[component]
pub fn Skills() -> impl IntoView {
    let skills = create_resource(
        || (),
        |_| async {
            Request::get("http://localhost:3000/api/skills")
                .send()
                .await
                .ok()?
                .json::<Vec<Skill>>()
                .await
                .ok()
        },
    );

    view! {
        <section id="skills" class="min-h-screen py-32 px-6 relative">
            <div class="container mx-auto max-w-6xl">
                <div class="text-center mb-20 animate-on-scroll">
                    <h2 class="text-5xl md:text-6xl font-bold gradient-text mb-4">
                        "Technical Expertise"
                    </h2>
                    <p class="text-xl text-slate-400 max-w-2xl mx-auto">
                        "Mastering blockchain technologies and Web3 development"
                    </p>
                </div>

                <Suspense fallback=move || view! {
                    <div class="grid md:grid-cols-2 gap-6">
                        {(0..6).map(|_| view! {
                            <div class="h-20 rounded-xl skeleton"></div>
                        }).collect::<Vec<_>>()}
                    </div>
                }>
                    {move || {
                        skills.get().and_then(|data| {
                            data.map(|skills| view! {
                                <div class="grid md:grid-cols-2 gap-6">
                                    {skills.into_iter().enumerate().map(|(i, skill)| view! {
                                        <div class="animate-on-scroll" style=format!("animation-delay: {}s", i as f32 * 0.1)>
                                            <SkillBar skill=skill/>
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
fn SkillBar(skill: Skill) -> impl IntoView {
    view! {
        <div class="glass-effect p-6 rounded-xl space-y-4 hover-lift">
            <div class="flex justify-between items-center">
                <span class="font-semibold text-lg">{skill.name}</span>
                <span class="text-cyan-400 font-mono text-sm">{skill.level}"%"</span>
            </div>
            <div class="h-3 bg-slate-800 rounded-full overflow-hidden">
                <div
                    class="skill-bar h-full bg-gradient-to-r from-cyan-500 to-blue-500 rounded-full"
                    style=format!("--skill-width: {}%", skill.level)
                />
            </div>
        </div>
    }
}
