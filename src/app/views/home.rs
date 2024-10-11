use leptos::{component, create_signal, view, IntoView, SignalUpdate};

#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <div class="flex flex-col items-center justify-center h-screen">
            <h1>"Welcome to Leptos!"</h1>
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" on:click=on_click>"Click Me: " {count}</button>
        </div>
    }
}
