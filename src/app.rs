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
        create_resource(cx, || (), |_| async { get_members().await });
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
        <AddMember/>
        <Suspense fallback=move || view! { cx, <p>"Loading members..."</p> }>
            <ul>{users_view}</ul>
        </Suspense>
    }
}

#[component]
fn AddMember(cx: Scope) -> impl IntoView {
    let add_member = create_server_multi_action::<AddMember>(cx);

    view! { cx,
        <MultiActionForm
            on:submit=move |ev| {
                let data = AddMember::from_event(&ev).expect("to parse form data");
                if data.username.contains(" ") {
                    ev.prevent_default();
                }
            }
            action=add_member
        >
            <label>
                "Add a member"
                <input type="text" name="username"/>
            </label>
            <input type="submit" value="Add"/>
        </MultiActionForm>
    }
}

#[derive(Params, Copy, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
    id: usize,
}

#[server(GetMembers, "/api", "Cbor")]
pub async fn get_members() -> Result<Vec<model::Member>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let members = crate::db::get_members().await?;
    Ok(members)
}

#[server(AddMember, "/api", "Cbor")]
pub async fn add_member(username: String) -> Result<(), ServerFnError> {
    Ok(crate::db::add_member(username).await?)
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
