use actix_web::{http::Method, rt::task, web, HttpRequest, HttpResponse};
use anyhow::{bail, Result};
use futures::{channel::oneshot, StreamExt};
use neon::prelude::*;
use std::{collections::HashMap, sync::Arc};
const MAX_SIZE: usize = 10_000;

pub async fn handle_request(
    function: Arc<Root<JsFunction>>,
    channel: web::Data<Channel>,

    payload: &mut web::Payload,
    req: &HttpRequest,
    route_params: HashMap<String, String>,
) -> HttpResponse {
    let contents = match execute_http_function(function, channel, payload, req, route_params).await
    {
        Ok(res) => res,
        Err(err) => {
            println!("Error: {:?}", err);
            let mut response = HttpResponse::InternalServerError();
            return response.finish();
        }
    };
    let mut response = HttpResponse::Ok();
    response.body(contents)
}

async fn execute_http_function(
    callback: Arc<Root<JsFunction>>,
    channel: web::Data<Channel>,
    payload: &mut web::Payload,
    req: &HttpRequest,
    route_params: HashMap<String, String>,
) -> Result<String> {
    let mut data: Option<Vec<u8>> = None;

    if req.method() == Method::POST {
        let mut body = web::BytesMut::new();
        while let Some(chunk) = payload.next().await {
            let chunk = chunk?;
            // limit max size of in-memory payload
            if (body.len() + chunk.len()) > MAX_SIZE {
                bail!("Body content Overflow");
            }
            body.extend_from_slice(&chunk);
        }

        data = Some(body.to_vec())
    }

    let mut request = HashMap::new();
    request.insert("params", route_params);
    let (sender, reciever) = oneshot::channel::<String>();
    task::spawn_blocking(move || {
        channel.send(move |mut cx| {
            let callback = callback.to_inner(&mut cx);
            let this = cx.undefined();
            let js_request = cx.empty_object();
            let params = request.get("params").unwrap();
            for (key, val) in params {
                let key = cx.string(key);
                let val = cx.string(val);
                js_request.set(&mut cx, key, val).unwrap();
            }
            match data {
                Some(res) => {
                    let buffer = JsArrayBuffer::external(&mut cx, res);
                    js_request.set(&mut cx, "body", buffer).unwrap();
                }
                None => {}
            }
            let args = vec![cx.null().upcast::<JsValue>(), js_request.upcast()];
            let result = callback.call(&mut cx, this, args)?;
            let result = result.to_string(&mut cx)?.value(&mut cx);
            sender.send(result).unwrap();
            Ok(())
        });
    });
    let ans = reciever.await;
    Ok(ans?)
}
