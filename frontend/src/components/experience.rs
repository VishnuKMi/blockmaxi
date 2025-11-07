use gloo_net::http::Request;
use leptos::*;
use shared::Experience as ExperienceData;

#[component]
pub fn Experience() -> impl IntoView {
    let experience = create_resource(
        || (),
        |_| async {
            Request::get("http://localhost:3000/api/experience")
                .send()
                .await
                .ok()?
                .json::<Vec<ExperienceData>>()
                .await
                .ok()
        },
    );

    view! {
        <section id="experience" class="min-h-screen py-20 px-6 bg-slate-900/50">
            <div class="container mx-auto">
                <h2 class="text-5xl font-bold text-center mb-16 bg-gradient-to-r from-cyan-400 to-blue-500 bg-clip-text text-transparent">
                    "Work Experience"
                </h2>
                <Suspense fallback=move || view! {
                    <div class="text-center text-slate-400">"Loading experience..."</div>
                }>
                    {move || {
                        experience.get().and_then(|data| {
                            data.map(|experiences| view! {
                                <div class="max-w-4xl mx-auto space-y-8">
                                    {experiences.into_iter().map(|exp| view! {
                                        <ExperienceCard experience=exp/>
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
fn ExperienceCard(experience: ExperienceData) -> impl IntoView {
    view! {
        <div class="bg-slate-800/50 p-8 rounded-xl border border-slate-700 hover:border-cyan-500 transition-all">
            <div class="flex justify-between items-start mb-4">
                <div>
                    <h3 class="text-2xl font-bold">{experience.role}</h3>
                    <p class="text-cyan-400 text-lg">{experience.company}</p>
                </div>
                <span class="text-slate-400">{experience.duration}</span>
            </div>
            <p class="text-slate-300 mb-4">{experience.description}</p>
            <ul class="space-y-2">
                {experience.achievements.into_iter().map(|achievement| view! {
                    <li class="flex items-start gap-2">
                        <span class="text-cyan-400 mt-1">"▸"</span>
                        <span class="text-slate-300">{achievement}</span>
                    </li>
                }).collect::<Vec<_>>()}
            </ul>
        </div>
    }
}
