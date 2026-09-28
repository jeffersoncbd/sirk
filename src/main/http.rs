pub(super) fn http(address: Option<&str>) -> Result<(), String> {
    sirk::http::serve(address.unwrap_or("127.0.0.1:8080"))
}
