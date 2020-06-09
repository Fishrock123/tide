#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    app.at("/").get(|_| async move { "Root".into() });
    app.at("/api").nest({
        let mut api = tide::new();
        api.at("/hello").get(|_| async move { "Hello, world".into() });
        api.at("/goodbye")
            .get(|_| async move { "Goodbye, world".into() });
        api
    });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
