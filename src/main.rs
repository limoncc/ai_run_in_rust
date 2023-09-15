
// region 一、解决模型运行问题
// fn main(){
//     println!("hello limoncc")
// }


// endregion



// region 二、解决api问题

use tracing::{info,Level};
use tracing_subscriber;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use std::time::Duration;
use axum::extract::Path;

#[tokio::main]
async fn main() {
    // 解决日志问题
    tracing_subscriber::fmt().with_max_level(Level::INFO) .with_target(false).init();
    info!("初始化化！");

    // 建立路由
    let app = Router::new()
        .route("/", get(root))
        .route("/sleep/:id", get(send_task_id));

    // 开启地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("监听: {addr}");
    //
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
// basic handler that responds with a static string
async fn root() -> &'static str {
    info!("Hello from tracing");
    tokio::time::sleep(Duration::from_secs(10)).await; // 休眠3秒，模拟超时
    info!("休眠了10秒哦！");
    "Hello, World!"

}

// 后台任务的实现，这个可以用来实现后台任务
async fn send_task_id(Path(timer): Path<i32>) -> String {
    tokio::spawn(async move { background_work(timer).await; });
    let task_id = blake3::hash(timer.to_string().as_bytes());
    format!("{{\"timer\": {timer},\"taskid\": {task_id}}}")
}

// 这个用来实现主要业务逻辑，需要长时间执行的任务
async fn background_work(timer: i32) {
    let task_id = blake3::hash(timer.to_string().as_bytes());
    info!("Start timer {timer}. task_id:{task_id}", );

    tokio::time::sleep(Duration::from_secs(30)).await;

    info!("Timer {timer} done. task_id:{task_id}");
}

// endregion