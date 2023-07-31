use crate::model;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/ssr_modes.css"/>
        <Title text="Welcome to Leptos"/>

        <h1>"main header"</h1>

        <Router>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=HomePage
                    />

                    <Route
                        path="/*any"
                        view=NotFound
                    />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let members =
        create_resource(cx, || (), |_| async { get_users().await });
    let users_view = move || {
        members.with(cx, |members| members
            .clone()
            .map(|members| {
                members.iter()
                .map(|m| view! { cx, <li>{&m.username}</li>})
                .collect_view(cx)
            })
        )
    };

    view! { cx,
        <h1>"homepage header"</h1>
        <Suspense fallback=move || view! { cx, <p>"Loading posts..."</p> }>
            <ul>{users_view}</ul>
        </Suspense>
    }
}

#[derive(Params, Copy, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
    id: usize,
}

#[server(GetUsers, "/api")]
pub async fn get_users() -> Result<Vec<model::Member>, ServerFnError> {
    let members = crate::db::get_members().await?;
    Ok(members)
}

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GetError {
    #[error("Invalid object ID.")]
    InvalidId,
    #[error("Object not found.")]
    PostNotFound,
    #[error("Server error.")]
    ServerError,
}

#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx, <h1>"Not Found"</h1> }
}
