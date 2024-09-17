use crate::{
    internals::*,
    logging::*,
    modules::header::{ContentType, TryIntoHeaderValue},
    pkg::{error::Error, helpers::*},
    types::http::*,
};

use http::header::*;
use std::{collections::HashMap, fmt, net::SocketAddr, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, Result as IoResult},
    net::{TcpListener, TcpStream},
};

impl Request {
    #[instrument]
    pub fn body(&self) -> &[u8] {
        debug!("request->body called");
        &self.body
    }

    #[instrument]
    pub fn body_length(&self) -> usize {
        debug!("request->body_length called");
        self.body.len()
    }

    #[instrument]
    pub fn method(&self) -> &Method {
        debug!("request->method called");
        &self.method
    }

    #[instrument]
    pub fn path(&self) -> &str {
        debug!("request->path called");
        self.path.as_str()
    }

    #[instrument]
    pub fn query(&self) -> &HashMap<String, String> {
        debug!("request->query called");
        &self.query
    }

    #[instrument]
    pub fn params(&self) -> &HashMap<String, String> {
        debug!("request->params called");
        &self.params
    }

    #[instrument]
    pub fn route_param(&self, name: &str) -> Option<&String> {
        debug!("request->trace called");
        self.params.get(name)
    }

    #[instrument]
    pub fn query_param(&self, name: &str) -> Option<&String> {
        debug!("request->query_param called");
        self.query.get(name)
    }

    #[instrument]
    pub fn header(&self, name: &str) -> Option<&HeaderValue> {
        debug!("request->trace called");
        self.headers.get(name)
    }

    #[instrument]
    pub fn is_json(&self) -> bool {
        debug!("request->is_json called");
        self.content_type().map(|ct| ct.0 == mime::APPLICATION_JSON).unwrap_or(false)
    }

    #[instrument]
    pub fn content_type(&self) -> Option<ContentType> {
        debug!("request->content_type called");
        self.header("content-type").and_then(|v| v.to_str().ok()).and_then(|s| s.parse::<mime::Mime>().ok()).map(ContentType)
    }

    #[instrument]
    pub fn text(&self) -> Result<String, Error> {
        debug!("request->text called");
        String::from_utf8(self.body.clone()).map_err(|e| Error(format!("Failed to parse body as text: {}", e)))
    }

    #[instrument]
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> Result<T, Error> {
        debug!("request->json called");
        serde_json::from_slice(&self.body).map_err(|e| Error(format!("Failed to parse body as JSON: {}", e)))
    }
}

impl Response {
    #[instrument]
    pub fn new() -> Self {
        trace!("response->new called");
        Response {
            path: String::new(),
            status: StatusCode::Ok,
            headers: HeaderMap::new(),
            body: Vec::new(),
        }
    }

    #[instrument]
    pub fn ok() -> Self {
        trace!("response->ok called");
        Self::new()
    }

    #[instrument]
    pub fn status(mut self, status: StatusCode) -> Self {
        trace!("response->status called");
        self.status = status;
        self
    }

    #[instrument]
    pub fn content_type<V>(&mut self, value: V) -> &mut Self
    where
        V: TryIntoHeaderValue + fmt::Debug,
    {
        trace!("response->content_type called");
        match value.try_into_value() {
            Ok(value) => {
                self.headers.insert(CONTENT_TYPE, value);
            }
            // Err(err) => self.error = Some(err.into()),
            Err(_) => self.status = StatusCode::from(500),
        };
        self
    }

    #[instrument]
    pub fn insert_header(mut self, header: (HeaderName, HeaderValue)) -> Self {
        trace!("response->insert_header called");
        self.headers.insert(header.0, header.1);
        self
    }

    #[instrument]
    pub fn body(mut self, body: impl Into<Vec<u8>> + fmt::Debug) -> Self {
        trace!("response->body called");
        self.body = body.into();
        self
    }

    #[instrument(skip(value))]
    pub fn json<T>(mut self, value: &T) -> Result<Self, serde_json::Error>
    where
        T: serde::Serialize,
    {
        trace!("response->json called");
        let body = serde_json::to_vec(value)?;
        self.content_type(ContentType::json());
        self.body = body;

        Ok(self)
    }

    #[instrument]
    pub fn redirect(mut self, status: StatusCode, location: &str) -> Result<Self, Error> {
        trace!("response->redirect called");
        self = crate::modules::redirect::create_redirect(self, status, location)?;
        Ok(self)
    }

    #[instrument]
    pub async fn write_headers<W>(&self, stream: &mut W) -> IoResult<()>
    where
        W: AsyncWriteExt + Unpin + fmt::Debug,
    {
        trace!("response->write_headers called");
        for (key, value) in self.headers.iter() {
            let header_name = key.as_str();
            let header_value = value
                .to_str()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid header value: {}", e)))?;

            let header = format!("{}: {}\r\n", header_name, header_value);
            stream.write_all(header.as_bytes()).await?;
        }
        Ok(())
    }
}

impl Router {
    pub fn new() -> Self { Router { routes: Vec::new() } }

    pub fn add<F>(&mut self, method: Method, path: String, handler: F) -> &mut Self
    where
        F: Fn(Request) -> HttpFuture + Send + Sync + 'static,
    {
        self.routes.push((method, path, Arc::new(handler)));
        self
    }

    pub fn add_default<F>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(Request) -> HttpFuture + Send + Sync + 'static,
    {
        self.routes.push((Method::ALL, String::default(), Arc::new(handler)));
        self
    }
}

impl Server {
    pub fn bind(host: &str, port: u16) -> Self {
        let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
        info!(port = port, host = host, "socket created");
        Server { addr }
    }

    pub async fn serve(self, router: Router) -> Result<(), Error> {
        let listener = TcpListener::bind(self.addr).await?;
        info!("starting {} workers", tokio::runtime::Handle::current().metrics().num_workers());

        loop {
            let (stream, _) = listener.accept().await?;
            let router = router.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, router).await {
                    error!("Error handling connection: {}", e);
                }
            });
        }
    }
}

async fn handle_connection(mut stream: TcpStream, router: Router) -> Result<(), Error> {
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let mut req = parse_request(&buffer[..n])?;

    let mut response = Response {
        path: req.path.to_owned(),
        status: StatusCode::NotFound,
        headers: HeaderMap::new(),
        body: b"Not Found".to_vec(),
    };

    for (method, path, handler) in router.routes.iter() {
        if req.method == *method && paths_match(&req.path, path) {
            req.params = extract_params(&req.path, path);

            match handler(req.clone()).await {
                Ok(responder) => {
                    response = responder.respond().await?;
                    break;
                }
                Err(e) => {
                    response = Response {
                        path: req.path.to_owned(),
                        status: StatusCode::InternalServerError,
                        headers: HeaderMap::new(),
                        body: format!("Internal Server Error: {}", e).into_bytes(),
                    };

                    break;
                }
            }
        }
    }

    if response.status == StatusCode::NotFound {
        for (method, _, handler) in router.routes.iter() {
            if *method == Method::ALL {
                match handler(req.clone()).await {
                    Ok(responder) => {
                        response = responder.respond().await?;
                        break;
                    }
                    Err(e) => {
                        response = Response {
                            path: req.path.to_owned(),
                            status: StatusCode::InternalServerError,
                            headers: HeaderMap::new(),
                            body: format!("Internal Server Error: {}", e).into_bytes(),
                        };

                        break;
                    }
                }
            }
        }
    }

    let response_string = format!(
        "\
        HTTP/1.1 {} {}\r\n\
        Server: Blaze HTTP\r\n\
        Content-Length: {}\r\n\
        Date: {}\r\n\
    ",
        response.status.to_code(),
        response.status.reason_phrase(),
        response.body.len(),
        crate::modules::date::now()
    );

    stream.write_all(response_string.as_bytes()).await?;
    response.write_headers(&mut stream).await?;

    stream.write_all(b"\r\n").await?;
    stream.write_all(&response.body).await?;

    let status_value = response.status.reason_phrase().to_lowercase();

    match response.status as u16 {
        200 | 201 | 204 | 301 | 302 | 303 | 304 | 307 => info!(path = req.path, method = req.method.to_string(), status = response.status.to_code(), "{status_value}"),
        400 | 401 | 403 | 404 | 405 => warn!(path = req.path, method = req.method.to_string(), status = response.status.to_code(), "{status_value}"),
        _ => error!(path = req.path, method = req.method.to_string(), status = response.status.to_code(), "{status_value}"),
    };

    Ok(())
}
