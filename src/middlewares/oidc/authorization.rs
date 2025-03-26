use rocket::http::Status;
use rocket::request::{ FromRequest, Outcome, Request };

pub struct Token {
    pub token: String,
}

fn extract_token_from_request(req: &Request<'_>) -> Option<String> {
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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = match extract_token_from_request(req) {
            Some(token) => token,
            None => {
                return Outcome::Error((Status::Unauthorized, ()));
            }
        };

        Outcome::Success(Token { token })
    }
}
