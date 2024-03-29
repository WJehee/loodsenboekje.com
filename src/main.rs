pub mod app;
pub mod components;
pub mod model;
pub mod auth;
pub mod errors;

use cfg_if::cfg_if;

cfg_if!{
    if #[cfg(feature = "ssr")] {
        use dotenvy::dotenv;
        use std::env;
        use std::fs::File;
        use axum_session::{Session, SessionConfig, SessionStore, SessionNullPool, SessionLayer, SecurityMode, Key};
        use axum::{
            Router,
            routing::get,
            response::{IntoResponse, Response},
            body::Body as AxumBody,
            extract::{Path, RawQuery, State},
            http::{Request, header::HeaderMap}
        };
        use leptos::*;
        use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
        use simplelog::*;

        use loodsenboekje::app::*;

        async fn server_fn_handler(
            session: Session<SessionNullPool>,
            path: Path<String>,
            headers: HeaderMap,
            raw_query: RawQuery,
            request: Request<AxumBody>
            ) -> impl IntoResponse {
            handle_server_fns_with_context(path, headers, raw_query, move || {
                provide_context(session.clone());
            }, request).await
        }

        #[axum::debug_handler]
        async fn leptos_routes_handler(session: Session<SessionNullPool>, State(leptos_options): State<LeptosOptions>, req: Request<AxumBody>) -> Response {
            let handler = leptos_axum::render_app_to_stream_with_context(leptos_options.clone(),
            move || {
                provide_context(session.clone());
            },
            || view! { <App/> }
            );
            handler(req).await.into_response()
        }

        fn routes_static(root: &str) -> axum::Router {
            use tower_http::services::ServeDir;
            use axum::routing::get_service;
            axum::Router::new().nest_service("/", get_service(ServeDir::new(root)))
        }

        #[tokio::main]
        async fn main() {
            // Use dotenv if available
            let _ = dotenv();
            env::var("READ_PASSWORD").expect("Expected READ_PASSWORD to be set");
            env::var("WRITE_PASSWORD").expect("Expected WRITE_PASSWORD to be set");
            env::var("ADMIN_PASSWORD").expect("Expected ADMIN_PASSWORD to be set");
            let data_dir = env::var("DATA_DIR").expect("Expected DATA_DIR to be set");

            let session_config = SessionConfig::default()
                .with_table_name("sessions")
                .with_key(Key::generate())
                .with_database_key(Key::generate())
                .with_security_mode(SecurityMode::PerSession);

            let session_store = SessionStore::<SessionNullPool>::new(None, session_config).await.expect("Failed to get session store");

            let conf = get_configuration(None).await.expect("Failed to get configuration");
            let leptos_options = conf.leptos_options;
            let addr = leptos_options.site_addr;
            let site_root = leptos_options.site_root.clone();
            let routes = generate_route_list(App);

            let router = Router::new()
                .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
                .leptos_routes_with_handler(routes, get(leptos_routes_handler))
                .with_state(leptos_options)
                .fallback_service(routes_static(&site_root))
                .layer(SessionLayer::new(session_store))
                ;
            let _ = CombinedLogger::init(vec![
                TermLogger::new(
                    LevelFilter::Debug,
                    ConfigBuilder::new()
                        .add_filter_allow("loodsenboekje".to_string())
                        .build(),
                    TerminalMode::Mixed,
                    ColorChoice::Auto,
                ),
                WriteLogger::new(
                    LevelFilter::Info,
                    ConfigBuilder::new()
                        .add_filter_allow("loodsenboekje".to_string())
                        .build(),
                    File::create(&format!("{data_dir}/loodsenboekje.log")).expect("Failed to open log file"),
                ),
            ]);

            axum::Server::bind(&addr).serve(router.into_make_service()).await.expect("Failed to start server");
        }
    } else {
        fn main() {}
    } 
}

