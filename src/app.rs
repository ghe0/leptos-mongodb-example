use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

use crate::{model, model::Member};

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
    let posts = create_resource(cx, || (), |_| async { get_posts().await });
    let posts_view = move || {
        posts.with(cx, |posts| {
            posts
                .clone()
                .map(|posts| posts.iter().map(|m| view! { cx, <li>{&m.text}</li>}).collect_view(cx))
        })
    };

    view! { cx,
        <h1>"homepage header"</h1>
        <Register/>
        <Login/>
        <Suspense fallback=move || view! { cx, <p>"Loading posts..."</p> }>
            <ul>{posts_view}</ul>
        </Suspense>
    }
}

#[component]
fn Register(cx: Scope) -> impl IntoView {
    let register = create_server_multi_action::<Register>(cx);
    view! { cx,
        <h3>Register</h3>
        <MultiActionForm
            on:submit=move |ev| {
                let data = Register::from_event(&ev).expect("to parse form data");
                if data.username.contains(" ") {
                    ev.prevent_default();
                }
            }
            action=register
        >
            <label>
                "Register"
                <input type="text" name="username"/>
                <input type="password" name="password"/>
            </label>
            <input type="submit" value="Add"/>
        </MultiActionForm>
    }
}

#[component]
fn Login(cx: Scope) -> impl IntoView {
    let login = create_server_multi_action::<Login>(cx);
    view! { cx,
        <h3>Login</h3>
        <MultiActionForm
            on:submit=move |ev| {
                let data = Login::from_event(&ev).expect("to parse form data");
                if data.username.contains(" ") {
                    ev.prevent_default();
                }
            }
            action=login
        >
            <label>
                "Login"
                <input type="text" name="username"/>
                <input type="password" name="password"/>
            </label>
            <input type="submit" value="Add"/>
        </MultiActionForm>
    }
}

#[derive(Params, Copy, Clone, Debug, PartialEq, Eq)]
pub struct PostParams {
    id: usize,
}

#[server(Register, "/api", "Cbor")]
pub async fn register(username: String, password: String) -> Result<(), ServerFnError> {
    Ok(crate::db::add_member(Member { username, password }).await?)
}

#[server(Login, "/api", "Cbor")]
pub async fn login(cx: Scope, username: String, password: String) -> Result<(), ServerFnError> {
    log!("logging in {username}");
    use actix_session::Session;
    use chrono::prelude::*;
    use leptos_actix::extract;
    #[derive(Serialize, Deserialize)]
    pub struct Token {
        username: String,
        exp: DateTime<Utc>,
    }
    crate::db::auth_member(&username, &password).await?;
    let exp = Utc::now().checked_add_signed(chrono::Duration::minutes(1)).unwrap();
    let token = Token { username, exp };

    let session = extract(cx, |session: Session| async move { session }).await.unwrap();
    session.insert("token", token)?;
    Ok(())
}

#[server(GetPosts, "/api", "Cbor")]
pub async fn get_posts() -> Result<Vec<model::Post>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let posts = crate::db::get_posts().await?;
    Ok(posts)
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
