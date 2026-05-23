mod battles;
mod health;

use rouille::{Request, Response};
use serde::Serialize;

pub fn handle_request(request: &Request) -> Response {
    if request.method() == "OPTIONS" {
        return cors(Response::empty_204());
    }

    let response = match request.method() {
        "GET" => route_get(request),
        _ => Response::empty_404(),
    };

    cors(response)
}

fn route_get(request: &Request) -> Response {
    let url = request.url();
    let path = url.split('?').next().unwrap_or(&url);

    if path == "/health" {
        return json_response(&health::health());
    }

    let Some(rest) = path.strip_prefix("/battles/") else {
        return Response::empty_404();
    };

    let segments: Vec<&str> = rest.split('/').collect();

    if segments.len() == 1 {
        if segments[0].is_empty() {
            return Response::empty_404();
        }
        return json_response(&battles::get_battles(segments[0]));
    }

    if segments.len() == 2 && segments[1] == "verify" {
        if segments[0].is_empty() {
            return Response::empty_404();
        }
        return json_response(&battles::verify_battle(segments[0]));
    }

    Response::empty_404()
}

fn cors(response: Response) -> Response {
    response
        .with_additional_header("Access-Control-Allow-Origin", "*")
        .with_additional_header("Access-Control-Allow-Methods", "GET, OPTIONS")
        .with_additional_header("Access-Control-Allow-Headers", "Content-Type")
}

fn json_response<T: Serialize>(value: &T) -> Response {
    match serde_json::to_string(value) {
        Ok(body) => Response::text(body)
            .with_additional_header("Content-Type", "application/json"),
        Err(_) => Response::text("{\"error\":\"serialization failed\"}")
            .with_status_code(500)
            .with_additional_header("Content-Type", "application/json"),
    }
}
