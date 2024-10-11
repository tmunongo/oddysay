use leptos::{component, server, view, IntoView, Params, ServerFnError, SignalWith};
use leptos_router::{use_params_map, Params};
use serde::{Serialize, Deserialize};
use crate::app::database::database::ssr::db;

#[derive(Deserialize, Serialize, Clone, Debug, Params, PartialEq)]
struct Query {
    chapter: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Chapter {
    slug: String,
    title: String,
    content: String,
}

#[server(ChapterPage, "/api/:slug")]
pub async fn get_chapter_page(query: Query) -> Result<Chapter, ServerFnError> {
    let mut conn = db.conn().await?;


}

#[component]
pub fn ChapterPage() -> impl IntoView {
    let params = use_params_map();

    let slug_fun = move || {
        params.with(|params| params.get("slug").cloned())
    };

    let chapter_fun = slug_fun();

    print!("Slug: {chapter_fun:?}");

    let chapter = match chapter_fun {
        Some(slug) => slug,
        None => return view! (
            <div class="flex flex-col items-center justify-center h-screen">
                <h1>404: No chapter found</h1>
            </div>
        )
    };

    print!("Slug: {chapter}");
    
    view! {
        <div class="flex flex-col items-center justify-center h-screen">
            <h1>Post</h1>
        </div>
    }
}