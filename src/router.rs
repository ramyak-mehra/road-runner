use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use actix_web::http::Method;
use matchit::Node;
use neon::prelude::*;

type RouteType = Arc<RwLock<Node<Arc<Root<JsFunction>>>>>;
impl Finalize for Router {}
pub type JsRouter = JsBox<Router>;

#[derive(Default)]
pub struct Router {
    get_routes: RouteType,
    post_routes: RouteType,
}

impl Router {
    fn get_map(&self, method: Method) -> Option<&RouteType> {
        match method {
            Method::GET => Some(&self.get_routes),
            Method::POST => Some(&self.post_routes),
            _ => None,
        }
    }

    fn add_route(&self, route_type: &str, route_path: &str, call_back: Root<JsFunction>) {
        let method = match Method::from_bytes(route_type.as_bytes()) {
            Ok(res) => res,
            Err(_) => return,
        };
        let table = match self.get_map(method) {
            Some(table) => table,
            None => return,
        };
        table
            .write()
            .unwrap()
            .insert(route_path, Arc::new(call_back))
            .expect("failed to insert route");
        println!("Route added for {} {} ", route_type, route_path);
    }
    pub fn get_route(
        &self,
        method: Method,
        route: &str,
    ) -> Option<(Arc<Root<JsFunction>>, HashMap<String, String>)> {
        let table = &mut self.get_map(method)?;
        match table.read().unwrap().at(route) {
            Ok(res) => {
                let mut route_params = HashMap::new();
                for (key, val) in res.params.iter() {
                    route_params.insert(key.to_string(), val.to_string());
                }
                Some((res.value.clone(), route_params))
            }
            Err(_) => None,
        }
    }
}

impl Router {
    pub fn js_add_route(mut cx: FunctionContext) -> JsResult<JsUndefined> {
        let js_router = cx.this().downcast_or_throw::<JsRouter, _>(&mut cx)?;
        let route_type = cx.argument::<JsString>(0)?.value(&mut cx);
        let route_path = cx.argument::<JsString>(1)?.value(&mut cx);
        let call_back = cx.argument::<JsFunction>(2)?.root(&mut cx);
        js_router.add_route(&route_type, &route_path, call_back);
        Ok(cx.undefined())
    }
    pub fn create_router(mut cx: FunctionContext) -> JsResult<JsRouter> {
        let router = Router::default();
        Ok(cx.boxed(router))
    }
}
