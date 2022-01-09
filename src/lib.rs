use neon::prelude::*;
mod server;
use crate::server::start_server;

fn get_num_cpus(mut cx: FunctionContext) -> JsResult<JsNumber> {
    Ok(cx.number(num_cpus::get() as f64))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("get", get_num_cpus)?;
    cx.export_function("startServer", start_server)?;
    

    Ok(())
}


