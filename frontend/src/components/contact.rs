use gloo_net::http::Request;
use leptos::*;
use shared::ContactForm;

#[component]
pub fn Contact() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (message, set_message) = create_signal(String::new());
    let (status, set_status) = create_signal(String::new());

    let submit_form = move |_| {
        let form = ContactForm {
            name: name.get(),
            email: email.get(),
            message: message.get(),
        };

        spawn_local(async move {
            let result = Request::post("http://localhost:3000/api/contact")
                .json(&form)
                .unwrap()
                .send()
                .await;

            match result {
                Ok(_) => {
                    set_status.set("Message sent successfully! ✓".to_string());
                    set_name.set(String::new());
                    set_email.set(String::new());
                    set_message.set(String::new());
                }
                Err(_) => {
                    set_status.set("Failed to send message. Please try again.".to_string());
                }
            }
        });
    };

    view! {
        <section id="contact" class="min-h-screen py-20 px-6">
            <div class="container mx-auto max-w-2xl">
                <h2 class="text-5xl font-bold text-center mb-16 bg-gradient-to-r from-cyan-400 to-blue-500 bg-clip-text text-transparent">
                    "Get In Touch"
                </h2>
                <div class="bg-slate-800/50 p-8 rounded-xl border border-slate-700 space-y-6">
                    <input
                        type="text"
                        placeholder="Your Name"
                        class="w-full px-4 py-3 bg-slate-900 border border-slate-700 rounded-lg focus:border-cyan-500 focus:outline-none transition-colors"
                        prop:value=name
                        on:input=move |e| set_name.set(event_target_value(&e))
                    />
                    <input
                        type="email"
                        placeholder="Your Email"
                        class="w-full px-4 py-3 bg-slate-900 border border-slate-700 rounded-lg focus:border-cyan-500 focus:outline-none transition-colors"
                        prop:value=email
                        on:input=move |e| set_email.set(event_target_value(&e))
                    />
                    <textarea
                        placeholder="Your Message"
                        rows="6"
                        class="w-full px-4 py-3 bg-slate-900 border border-slate-700 rounded-lg focus:border-cyan-500 focus:outline-none transition-colors resize-none"
                        prop:value=message
                        on:input=move |e| set_message.set(event_target_value(&e))
                    />
                    <button
                        on:click=submit_form
                        class="w-full px-8 py-3 bg-gradient-to-r from-cyan-500 to-blue-500 rounded-lg hover:scale-105 transition-transform font-semibold"
                    >
                        "Send Message"
                    </button>
                    {move || {
                        let status_text = status.get();
                        if !status_text.is_empty() {
                            view! {
                                <p class="text-center text-cyan-400">{status_text}</p>
                            }.into_view()
                        } else {
                            view! { <span/> }.into_view()
                        }
                    }}
                </div>
            </div>
        </section>
    }
}
