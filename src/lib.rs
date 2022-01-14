use neon::prelude::*;
mod processor;
mod router;
mod server;
use crate::server::Server;
pub use processor::handle_request;
pub use router::{JsRouter, Router};

fn get_num_cpus(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(num_cpus::get() as f64))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get", get_num_cpus)?;
    cx.export_function("createServer", Server::new)?;
    cx.export_function("startServer", Server::start_server)?;
    cx.export_function("createRouter", Router::create_router)?;
    cx.export_function("addRoute", Router::js_add_route)?;
    Ok(())
}
