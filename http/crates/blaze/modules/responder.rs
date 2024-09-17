use crate::{
    internals::RespFuture,
    modules::header::ContentType,
    types::alias::*,
    types::http::{Responder, Response},
};

use serde::Serialize;

impl Responder for Response {
    fn respond(self: Box<Self>) -> RespFuture { Box::pin(async move { Ok(*self) }) }
}

impl<T: Serialize + Send + 'static> Responder for Json<T> {
    fn respond(self: Box<Self>) -> RespFuture { Box::pin(async move { Ok(Response::ok().json(&self.0)?.content_type(ContentType::json()).into()) }) }
}

impl Responder for String {
    fn respond(self: Box<Self>) -> RespFuture { Box::pin(async move { Ok(Response::ok().body(self.into_bytes()).content_type(ContentType::plaintext()).into()) }) }
}

impl Responder for Vec<u8> {
    fn respond(self: Box<Self>) -> RespFuture { Box::pin(async move { Ok(Response::ok().body(*self).content_type(ContentType::plaintext()).into()) }) }
}

impl Responder for &'static str {
    fn respond(self: Box<Self>) -> RespFuture { Box::pin(async move { Ok(Response::ok().body(self.as_bytes().to_vec()).content_type(ContentType::plaintext()).into()) }) }
}

impl Responder for &'static [u8] {
    fn respond(self: Box<Self>) -> RespFuture { Box::pin(async move { Ok(Response::ok().body(self.to_vec()).content_type(ContentType::plaintext()).into()) }) }
}
