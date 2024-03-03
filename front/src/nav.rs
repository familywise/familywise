fn api_endpoint() -> String {
    let window = web_sys::window().expect("No window found.");
    let location = window.location();
    let host = location.host().expect("No host found.");
    let protocol = location.protocol().expect("No protocol found.");
    format!("{}//{}", protocol, host)
}

pub fn guests_endpoint() -> String {
    format!("{}/guests", api_endpoint())
}

pub fn improv_endpoint() -> String {
    format!("{}/improv", api_endpoint())
}
