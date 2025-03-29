use rocket::Request;

pub fn from_header(req: &Request<'_>) -> Option<String> {
    let authorization = req.headers().get_one("Authorization");
    if authorization.is_none() {
        return None;
    }

    let authorization = authorization.unwrap();
    if !authorization.starts_with("Bearer ") {
        return None;
    }
    let token = authorization[7..].to_string();

    Some(token)
}
