// -*- coding: utf-8 -*-
// ==============================
// 作者: 李健/limoncc
// 邮箱: limoncc@icloud.com
// 日期：2023/9/15 17:13
// 标题：ai run in rust
// 内容：解决rust做模型服务的问题
// ==============================


// 模型库
use tch::jit;
use tch::Tensor;
// 日志库
use tracing::{info, Level};
use tracing_subscriber;
// api库
use axum::Json;
use axum::Router;
use axum::extract::{State};
use axum::routing::{get, post};
use std::net::SocketAddr;
// use std::time::Duration;

// 处理数据
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

// 文档库
use utoipa::OpenApi;
use utoipa::ToSchema;
use utoipa_rapidoc::RapiDoc;
// use utoipa_swagger_ui::SwaggerUi;
// region 一、全面解决

// featurs and issues ：
// 1、暂时无法解决模型文件打包问题，通过命令行参数外部设定解决
// 2、支持多线程并发
// 3、支持后台任务
// 4、无需python环境
// 5、一次编译到处运行
// 6、即将支持sdxl，baichaun2等大模型


// 状态共享
struct Ai {
    model: jit::CModule,
    call_times: i64,
}

impl Ai {
    fn add(&mut self) {
        let old_times = &self.call_times;
        self.call_times = old_times + 1
    }
}

#[utoipa::path(
get,
path = "/hello",
responses((status = 200, description = "执行模型推导", body = str)),
tag = "模型",
)]
async fn hello() -> &'static str {
    info!("Hello, World!");
    "Hello, World!"
}

#[derive(Deserialize, ToSchema)]
struct AiInputs {
    #[schema(example = json!([1,2,3,4,5]))]
    data: Vec<f32>,
}

#[derive(Serialize, ToSchema)]
struct AiOutputs {
    #[schema(example = json!([0.5774556, 0.42254442]))]
    result: Vec<f32>,
}


#[utoipa::path(
post,
path = "/infer",
request_body = AiInputs,
responses((status = 200, description = "执行模型推导", body = AiOutputs)),
tag = "模型"
)]
async fn inference(State(ai): State<Arc<Mutex<Ai>>>, Json(payload): Json<AiInputs>) -> Json<AiOutputs> {
    let inputs = Tensor::from_slice(&payload.data);
    let outputs_tensor = ai.clone().lock().unwrap().model.forward_ts(&[inputs]).unwrap();
    // tensor转换为原生vec
    let outputs = Vec::<f32>::try_from(outputs_tensor).unwrap();
    ai.clone().lock().unwrap().add();
    let nums = ai.clone().lock().unwrap().call_times;
    info!("第{nums}次调用了模型输出{outputs:?}");
    let data = AiOutputs { result: outputs };
    // 转换为 JSON response 输出
    Json(data)
}


#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
    paths(inference, hello),
    components(schemas(AiInputs, AiOutputs)),
    tags(
    (name = "模型", description = "这是执行模型推断的API")
    )
    )]
    struct ApiDoc;

    // 建立文档
    let apidoc = ApiDoc::openapi();
    let doc = RapiDoc::with_openapi("/api-docs/openapi.json", apidoc).path("/docs");
    // let doc = SwaggerUi::new("/docs").url("/api-docs/openapi.json", apidoc);


    // 解决日志问题
    tracing_subscriber::fmt().with_max_level(Level::INFO).with_target(false).init();
    info!("这里是 AI run in rust 项目");
    let model = jit::CModule::load("/Users/xiaobai/dev/ai_run_in_rust/data/model.jit").unwrap();
    let ai = Arc::new(Mutex::new(Ai { model: model, call_times: 0 }));
    info!("初始化模型完毕！");

    // 建立路由
    let app = Router::new()
        .merge(doc)
        .route("/",get(hello))
        .route("/hello", get(hello))
        .route("/infer", post(inference))
        .with_state(ai);

    // 设置地址
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("监听: {addr}");
    // 开启服务
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}


// endregion


// region 二、解决模型运行问题
// 静态资源
// use rust_embed::RustEmbed;
// #[derive(RustEmbed)]
// #[folder = "data/"]
// struct Asset;
// use tch::jit;
// use tch::Tensor;
//
// fn main() {
//     println!("hello limoncc");
// // 张量计算
// let t = Tensor::from_slice(&[3, 1, 4, 1, 5]);
// let t = t * 2;
// // t.print();
// println!("{:}", t);
//
// // 梯度计算
// let mut x = Tensor::from(2.0f32).to(Device::Mps).set_requires_grad(true);
// let y = &x * &x + &x + 36;
// println!("y {}", y.double_value(&[]));
//
// x.zero_grad();
// y.backward();
//
// let dy_over_dx = x.grad();
// println!("dy/dx {}", dy_over_dx.double_value(&[]));
// 下面来个模型导出并在rust中使用的例子
// 载入模型
// let model = jit::CModule::load("/Users/xiaobai/dev/ai_run_in_rust/data/model.jit").unwrap();
// let inputs = Tensor::from_slice(&[1f32, 2f32, 3f32, 4f32, 5f32]);
// let outputs = model.forward_ts(&[inputs]).unwrap();
// println!("{:}", outputs); // tensor([0.4980, 0.5020])
// let a = Vec::<f32>::try_from(outputs).unwrap();
// println!("{a:?}");

// }


// endregion


// region 三、解决api问题

// use tracing::{info,Level};
// use tracing_subscriber;
// use axum::{routing::get, Router};
// use std::net::SocketAddr;
// use std::time::Duration;
// use axum::extract::Path;
//
// #[tokio::main]
// async fn main() {
//     // 解决日志问题
//     tracing_subscriber::fmt().with_max_level(Level::INFO) .with_target(false).init();
//     info!("初始化化！");
//
//     // 建立路由
//     let app = Router::new()
//         .route("/", get(root))
//         .route("/sleep/:id", get(send_task_id));
//
//     // 开启地址
//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     info!("监听: {addr}");
//     //
//     axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
// }
// // basic handler that responds with a static string
// async fn root() -> &'static str {
//     info!("Hello from tracing");
//     tokio::time::sleep(Duration::from_secs(10)).await; // 休眠3秒，模拟超时
//     info!("休眠了10秒哦！");
//     "Hello, World!"
//
// }
//
// // 后台任务的实现，这个可以用来实现后台任务
// async fn send_task_id(Path(timer): Path<i32>) -> String {
//     tokio::spawn(async move { background_work(timer).await; });
//     let task_id = blake3::hash(timer.to_string().as_bytes());
//     format!("{{\"timer\": {timer},\"taskid\": {task_id}}}")
// }
//
// // 这个用来实现主要业务逻辑，需要长时间执行的任务
// async fn background_work(timer: i32) {
//     let task_id = blake3::hash(timer.to_string().as_bytes());
//     info!("Start timer {timer}. task_id:{task_id}", );
//
//     tokio::time::sleep(Duration::from_secs(30)).await;
//
//     info!("Timer {timer} done. task_id:{task_id}");
// }

// 还有一个问题没有解决，中间状态问题。

// endregion