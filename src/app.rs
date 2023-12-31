use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::model::{Member, Post};

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
        <Post/>
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

#[component]
fn Post(cx: Scope) -> impl IntoView {
    let add_post = create_server_multi_action::<AddPost>(cx);
    view! { cx,
        <h3>Add a post</h3>
        <MultiActionForm
            action=add_post
        >
            <label>
                "post text"
                <input type="text" name="text"/>
            </label>
            <input type="submit" value="Add"/>
        </MultiActionForm>
    }
}

#[server(Register, "/api")]
pub async fn register(username: String, password: String) -> Result<(), ServerFnError> {
    Ok(crate::db::add_member(Member { username, password }).await?)
}

#[server(Login, "/api")]
pub async fn login(cx: Scope, username: String, password: String) -> Result<(), ServerFnError> {
    crate::db::auth_member(&username, &password).await?;
    crate::auth::login(cx, username).await?;
    Ok(())
}

#[server(GetPosts, "/api")]
pub async fn get_posts() -> Result<Vec<Post>, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let posts = crate::db::get_posts().await?;
    Ok(posts)
}

#[server(AddPost, "/api")]
pub async fn add_post(cx: Scope, text: String) -> Result<(), ServerFnError> {
    crate::auth::verify(cx).await?;
    crate::db::add_post(Post { text }).await?;
    Ok(())
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
