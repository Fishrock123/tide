use async_std::task;
use tide::http::Cookie;
use tide::{Request, StatusCode};

/// Tide will use the the `Cookies`'s `Extract` implementation to build this parameter.
///
async fn retrieve_cookie(cx: Request<()>) -> tide::Response {
    format!("hello cookies: {:?}", cx.cookie("hello").unwrap()).into()
}

async fn insert_cookie(_req: Request<()>) -> tide::Response {
    let mut res = tide::Response::new(StatusCode::Ok);
    res.insert_cookie(Cookie::new("hello", "world"));
    res
}

async fn remove_cookie(_req: Request<()>) -> tide::Response {
    let mut res = tide::Response::new(StatusCode::Ok);
    res.remove_cookie(Cookie::named("hello"));
    res
}

fn main() -> Result<(), std::io::Error> {
    task::block_on(async {
        let mut app = tide::new();

        app.at("/").get(retrieve_cookie);
        app.at("/set").get(insert_cookie);
        app.at("/remove").get(remove_cookie);
        app.listen("127.0.0.1:8080").await?;

        Ok(())
    })
}
