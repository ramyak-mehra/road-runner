use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use neon::prelude::*;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn run_server(port:  usize) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}

pub fn start_server(mut cx: FunctionContext) -> JsResult<JsNull> {
    let port = cx.argument::<JsNumber>(0)?.value(&mut cx) as usize;

    actix_web::rt::System::new("server1".to_string())
        .block_on(run_server(port))
        .unwrap();

    Ok(cx.null())
}
