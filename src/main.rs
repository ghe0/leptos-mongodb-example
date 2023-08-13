#![cfg(feature = "ssr")]

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_session::{
        config::PersistentSession, storage::CookieSessionStore, SessionMiddleware,
    };
    use actix_web::{
        cookie::{time::Duration, Key},
        web, App, HttpServer,
    };
    use base64::{engine::general_purpose, Engine as _};
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_mongodb::app::*;

    const ONE_MINUTE: Duration = Duration::minutes(1);

    leptos_mongodb::db::init().await;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> });

    let secret_key = std::env::var("SECRET_KEY").expect("Please supply the SECRET_KEY env var.");
    let secret_key = Key::from(&general_purpose::STANDARD.decode(secret_key).unwrap());
    let encoded_key = general_purpose::STANDARD.encode(secret_key.master());
    log!("The secret is {encoded_key}");

    HttpServer::new(move || {
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", site_root))
            .service(favicon)
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("rust-test".to_owned())
                    .cookie_secure(false)
                    .session_lifecycle(PersistentSession::default().session_ttl(ONE_MINUTE))
                    .build(),
            )
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), App)
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!("{site_root}/favicon.ico"))?)
}
