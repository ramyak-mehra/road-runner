use std::{collections::HashMap, sync::Arc, thread};

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::channel::oneshot;
use neon::handle::Root;
use neon::prelude::*;

use crate::{handle_request, JsRouter};
type JsServer = JsBox<Server>;
struct Server {
    router: Arc<Root<JsRouter>>,
}
impl Server {
    fn new(mut cx: FunctionContext) -> Self {
        let router = cx
            .argument::<JsRouter>(0)
            .expect("no router provided")
            .root(&mut cx);
        Self {
            router: Arc::new(router),
        }
    }

pub fn start_server(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let server = cx.this().downcast_or_throw::<JsServer, _>(&mut cx)?;
    let router = server.get_router(&mut cx);
    let channel = cx.channel();
    thread::spawn(move || {
        actix_web::rt::System::new().block_on(async move {
            println!("Starting server");
            HttpServer::new(move || {
                let mut app = App::new();
                app = app.app_data(web::Data::new(router.clone()));
                app = app.app_data(web::Data::new(channel.clone()));
                app.default_service(
                    web::route().to(move |channel, router, payload, req| {
                        index(channel, router, payload, req)
                    }),
                )
            })
            .bind("127.0.0.1:8080")
            .unwrap()
            .run()
            .await
            .unwrap()
        });
    });
    Ok(cx.undefined())
}
}

impl Server {
    fn get_router<'a, C>(&self, cx: &mut C) -> Arc<Root<JsRouter>>
    where
        C: Context<'a>,
    {
        Arc::clone(&self.router)
    }
}


/// This is our service handler. It receives a Request, routes on it
/// path, and returns a Future of a Response.
async fn index(
    channel: web::Data<Channel>,
    router: web::Data<Arc<Root<JsRouter>>>,
    mut payload: web::Payload,
    req: HttpRequest,
) -> impl Responder {
    let mut queries = HashMap::new();

    if req.query_string().len() > 0 {
        let split = req.query_string().split("&");
        for s in split {
            let params = s.split_once("=").unwrap_or((s, ""));
            queries.insert(params.0, params.1);
        }
    }
    let (sender, reciever) =
        oneshot::channel::<Option<(Arc<Root<JsFunction>>, HashMap<String, String>)>>();
    let method = req.method().clone();
    channel.send(move |mut cx| {
        let router = router.to_inner(&mut cx);
        sender.send(router.get_route(method, "path")).unwrap();
        Ok(())
    });
    let result = reciever.await.expect("failed to get router");

    match result {
        Some((handler_function, route_params)) => {
            handle_request(handler_function, channel, &mut payload, &req, route_params).await
        }
        None => {
            let mut response = HttpResponse::Ok();
            response.finish()
        }
    }
}

#[derive(Default)]
pub struct FunctionsHandler {
    js_callback: Option<Root<JsFunction>>,
}
impl FunctionsHandler {
    pub fn js_new(mut cx: FunctionContext) -> JsResult<JsBox<FunctionsHandler>> {
        let route_type = cx.argument::<JsString>(0)?.value(&mut cx);
        let route_path = cx.argument::<JsString>(1)?.value(&mut cx);
        let call_back = cx.argument::<JsFunction>(2)?.root(&mut cx);
        println!("Route added for {} {} ", route_type, route_path);

        let fh = FunctionsHandler {
            js_callback: Some(call_back),
        };
        Ok(cx.boxed(fh))
    }
    // fn js_add_route(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    //     let fh = cx
    //         .this()
    //         .downcast_or_throw::<JsBox<FunctionsHandler>, _>(&mut cx)?;
    //     let route_type = cx.argument::<JsString>(0)?.value(&mut cx);
    //     let route_path = cx.argument::<JsString>(1)?.value(&mut cx);
    //     let call_back = cx.argument::<JsFunction>(2)?.root(&mut cx);
    //     let f = fh.root(&mut cx).to_inner(&mut cx);
    //     // f.add_route(&route_type, &route_path, call_back);
    //     Ok(cx.undefined())
    // }
}
impl FunctionsHandler {
    fn get_callback<'a, C>(&self, cx: &mut C) -> Root<JsFunction>
    where
        C: Context<'a>,
    {
        self.js_callback
            .as_ref()
            .expect("no callback added")
            .clone(cx)
    }
}
impl Finalize for FunctionsHandler {}

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

async fn run_server(port: usize) -> std::io::Result<()> {
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

// pub fn start_server(mut cx: FunctionContext) -> JsResult<JsNull> {
//     let port = cx.argument::<JsNumber>(0)?.value(&mut cx) as usize;

//     actix_web::rt::System::new()
//         .block_on(run_server(port))
//         .unwrap();

//     Ok(cx.null())
// }
impl Finalize for Server {}

// async fn index(
//     channel: web::Data<Channel>,
//     router: web::Data<Router>,
//     mut payload: web::Payload,
//     req: HttpRequest,
// ) -> impl Responder {
//     let mut queries = HashMap::new();

//     if req.query_string().len() > 0 {
//         let split = req.query_string().split("&");
//         for s in split {
//             let params = s.split_once("=").unwrap_or((s, ""));
//             queries.insert(params.0, params.1);
//         }
//     }
//     handle_request(
//         handler_function,
//         channel,
//         &mut payload,
//         &req,
//         route_params,
//         queries,
//     )
//     .await

// match router
//     .get_route(channel, req.method().clone(), req.uri().path())
//     .await
// {
//     Some((handler_function, route_params)) => {
//         handle_request(
//             handler_function,
//             channel,
//             &mut payload,
//             &req,
//             route_params,
//             queries,
//         )
//         .await
//     }
//     None => {
//         let mut response = HttpResponse::Ok();
//         response.finish()
//     }
// }
// }
