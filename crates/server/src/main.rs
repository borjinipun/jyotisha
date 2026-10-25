//! `jyotisha-server` binary entry point.

#[tokio::main]
async fn main() {
    let mut port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    let mut bind = "127.0.0.1".to_string();
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--port" => {
                if let Some(v) = args.next() {
                    port = v.parse().expect("port must be a number");
                }
            }
            "--bind" => {
                if let Some(v) = args.next() {
                    bind = v;
                }
            }
            other => {
                eprintln!(
                    "unknown argument: {other} (usage: jyotisha-server [--port N] [--bind HOST])"
                );
                std::process::exit(2);
            }
        }
    }
    let addr = std::net::SocketAddr::new(
        bind.parse().expect("invalid bind address"),
        port,
    );
    let listener = tokio::net::TcpListener::bind(addr).await.expect("bind");
    eprintln!("jyotisha-server listening on http://{addr}");
    axum::serve(listener, jyotisha_server::app()).await.expect("serve");
}
