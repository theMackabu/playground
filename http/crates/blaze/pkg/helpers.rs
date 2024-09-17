use crate::{pkg::error::Error, types::http::*};

use http::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;

pub fn parse_query_string(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter(|s| !s.is_empty())
        .filter_map(|s| {
            let mut parts = s.splitn(2, '=');
            Some((parts.next()?.to_string(), parts.next().unwrap_or("").to_string()))
        })
        .collect()
}

pub fn extract_params(request_path: &str, route_path: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    let req_segments: Vec<&str> = request_path.split('/').collect();
    let route_segments: Vec<&str> = route_path.split('/').collect();

    for (req_seg, route_seg) in req_segments.iter().zip(route_segments.iter()) {
        if route_seg.starts_with('{') && route_seg.ends_with('}') {
            let param_name = &route_seg[1..route_seg.len() - 1];
            params.insert(param_name.to_string(), req_seg.to_string());
        }
    }

    params
}

pub fn paths_match(request_path: &str, route_path: &str) -> bool {
    let req_segments: Vec<&str> = request_path.split('/').collect();
    let route_segments: Vec<&str> = route_path.split('/').collect();

    if req_segments.len() != route_segments.len() {
        return false;
    }

    for (req_seg, route_seg) in req_segments.iter().zip(route_segments.iter()) {
        if !route_seg.starts_with('{') && req_seg != route_seg {
            return false;
        }
    }

    true
}

pub fn parse_headers<'a, I>(lines: I) -> Result<HeaderMap, Error>
where
    I: Iterator<Item = &'a str>,
{
    let mut headers = HeaderMap::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(':') {
            let header_name = HeaderName::from_bytes(key.trim().as_bytes()).map_err(|e| Error(format!("Invalid header name: {}", e)))?;
            let header_value = HeaderValue::from_str(value.trim()).map_err(|e| Error(format!("Invalid header value: {}", e)))?;
            headers.insert(header_name, header_value);
        }
    }
    Ok(headers)
}

pub fn parse_request(buffer: &[u8]) -> Result<Request, Error> {
    let request_str = String::from_utf8_lossy(buffer);
    let mut parts = request_str.splitn(2, "\r\n\r\n");

    let headers_part = parts.next().ok_or_else(|| Error("Invalid request".into()))?;
    let body_part = parts.next().unwrap_or("");

    let mut lines = headers_part.lines();
    let first_line = lines.next().ok_or_else(|| Error("Invalid request".into()))?;
    let mut parts = first_line.split_whitespace();

    let method = match parts.next() {
        Some("GET") => Method::GET,
        Some("POST") => Method::POST,
        Some("PUT") => Method::PUT,
        Some("DELETE") => Method::DELETE,
        _ => return Err(Error("Invalid method".into())),
    };

    let full_path = parts.next().ok_or_else(|| Error("Invalid path".into()))?;
    let (path, query) = full_path.split_once('?').unwrap_or((full_path, ""));
    let query_params = parse_query_string(query);

    let headers = parse_headers(lines)?;
    let body = body_part.as_bytes().to_vec();

    Ok(Request {
        method,
        headers,
        query: query_params,
        body,
        params: HashMap::new(),
        path: path.to_string(),
    })
}
