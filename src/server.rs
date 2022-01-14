use std::{collections::HashMap, sync::Arc, thread};

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::channel::oneshot;
use neon::handle::Root;
use neon::prelude::*;

use crate::{handle_request, JsRouter};

impl Finalize for Server {}

type JsServer = JsBox<Server>;
pub struct Server {
    router: Arc<Root<JsRouter>>,
}
impl Server {
    pub fn new(mut cx: FunctionContext) -> JsResult<JsServer> {
        let router = cx
            .argument::<JsRouter>(0)
            .expect("no router provided")
            .root(&mut cx);
        let js_server = cx.boxed(Self {
            router: Arc::new(router),
        });
        Ok(js_server)
    }

    pub fn start_server(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let server = cx.this().downcast_or_throw::<JsServer, _>(&mut cx)?;
        let port = cx.argument::<JsNumber>(0)?.value(& mut cx);
        let router = server.get_router();
        let channel = cx.channel();
        thread::spawn(move || {
            actix_web::rt::System::new().block_on(async move {
                println!("Starting server");
                HttpServer::new(move || {
                    let mut app = App::new();
                    app = app.app_data(web::Data::new(router.clone()));
                    app = app.app_data(web::Data::new(channel.clone()));
                    app.default_service(web::route().to(move |channel, router, payload, req| {
                        index(channel, router, payload, req)
                    }))
                })
                .bind(format!("127.0.0.1:{}",port))
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
    fn get_router(&self) -> Arc<Root<JsRouter>> {
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

    let path = req.path().to_owned();
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
        sender.send(router.get_route(method, &path)).unwrap();
        Ok(())
    });
    let result = reciever.await.expect("failed to get router");

    match result {
        Some((handler_function, route_params)) => {
            handle_request(handler_function, channel, &mut payload, &req, route_params).await
        }
        None => {
            let mut response = HttpResponse::NotFound();
            response.finish()
        }
    }
}
