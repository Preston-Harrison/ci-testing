use warp::Filter;

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PATH").unwrap().parse().unwrap();
    let hello_world = warp::path!("/hello" / String).map(|name| format!("Hello, {}", name));
    warp::serve(hello_world).run(([0, 0, 0, 0], port)).await
}
