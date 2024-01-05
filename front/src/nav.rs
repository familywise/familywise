fn api_endpoint() -> String {
    let window = web_sys::window().expect("No window found.");
    let location = window.location();
    let host = location.host().expect("No host found.");
    let protocol = location.protocol().expect("No protocol found.");
    let api = "api".to_string();
    format!("{}//{}/{}", protocol, host, api)
}

pub fn users_endpoint() -> String {
    format!("{}/users", api_endpoint())
}
