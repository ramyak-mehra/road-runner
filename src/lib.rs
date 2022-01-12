use neon::prelude::*;
mod processor;
mod router;
mod server;
use crate::server::{start_server, FunctionsHandler};
pub use processor::handle_request;
use router::{add_get_route, add_post_route, create_router};
pub use router::{JsRouter, Router};

fn get_num_cpus(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(num_cpus::get() as f64))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get", get_num_cpus)?;
    cx.export_function("startServer", start_server)?;
    cx.export_function("functionsHandlerNew", FunctionsHandler::js_new)?;
    cx.export_function("createRouter", add_get_route)?;
    cx.export_function("addGetRoute", add_get_route)?;
    cx.export_function("addPostRoute", add_get_route)?;
    Ok(())
}
