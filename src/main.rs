use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello_world = warp::get().map(|| String::from("Hello World with update!"));

    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let sum = warp::path!("sum" / u32 / u32).map(|a, b| format!("{} + {} = {}", a, b, a + b));

    println!("Starting Server on port 8000");

    warp::serve(hello.or(sum).or(hello_world))
        .run(([0, 0, 0, 0], 8000))
        .await;
}
