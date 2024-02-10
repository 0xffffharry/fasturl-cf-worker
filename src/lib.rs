use std::str::FromStr;

use worker::*;

#[event(fetch)]
async fn main(req: Request, _: Env, _: Context) -> Result<Response> {
    let req_uri = match req.url() {
        Ok(uri) => uri,
        Err(e) => return Response::error(format!("invalid uri: {}", e), 400),
    };
    let mut sub_req_uri_str = req_uri.path().trim_start_matches('/').to_string();
    if let Some(query) = req_uri.query() {
        sub_req_uri_str.push_str("?");
        sub_req_uri_str.push_str(query);
    }
    if let Some(fragment) = req_uri.fragment() {
        sub_req_uri_str.push_str("#");
        sub_req_uri_str.push_str(fragment);
    }
    let uri = match url::Url::from_str(&sub_req_uri_str) {
        Ok(uri) => uri,
        Err(e) => return Response::error(format!("invalid uri: {}", e), 400),
    };
    console_debug!("fetching: {}", &uri);
    let mut sub_req = Request::new(&uri.to_string(), req.method())?;
    {
        let sub_req_headers = sub_req.headers_mut()?;
        req.headers().entries().for_each(|(k, v)| {
            let _ = sub_req_headers.append(&k, &v);
        });
    }
    let sub_fetch = Fetch::Request(sub_req);
    let sub_response = sub_fetch.send().await?;
    Ok(sub_response)
}
