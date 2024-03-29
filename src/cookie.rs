use axum::http::HeaderMap;

const COOKIE_NAME: &str = "blog_admin";

pub fn get_cookie(headers: &HeaderMap) -> Option<String> {
    let cookie = headers
        .get(axum::http::header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string());

    match cookie {
        Some(cookie) => {
            let cookie = cookie.as_str();
            let cs: Vec<&str> = cookie.split(";").collect();
            for item in cs {
                let item: Vec<&str> = item.split("=").collect();
                if item.len() != 2 {
                    continue;
                }
                let key = item[0].trim();
                let value = item[1].trim();
                if key == COOKIE_NAME {
                    return Some(value.to_string());
                }
            }
            None
        }
        None => None,
    }
}

pub fn set_cookie(value: &str) -> HeaderMap {
    let c = format!("{}={}", COOKIE_NAME, value);
    let mut hm = HeaderMap::new();
    hm.insert(axum::http::header::SET_COOKIE, (&c).parse().unwrap());
    hm
}
