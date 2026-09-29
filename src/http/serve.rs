use super::routes::routes;

pub fn serve(address: &str) -> Result<(), String> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|error| error.to_string())?
        .block_on(async {
            let listener = tokio::net::TcpListener::bind(address)
                .await
                .map_err(|error| error.to_string())?;
            eprintln!("S.I.R.K. HTTP server listening on http://{address}");
            axum::serve(listener, routes())
                .await
                .map_err(|error| error.to_string())
        })
}
