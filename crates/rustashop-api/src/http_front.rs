//! Serenade HTTP front controller (commerce routes migrate onto this over time).

use serenade_http::{
    box_future, AsyncHttpKernel, HttpError, Method, Request, Response, Route, RouteCollection,
    UrlMatcher,
};

use crate::health::health_json_body;

const HEALTHZ_ROUTE: &str = "healthz";

/// Builds the Serenade async kernel used for HTTP that has already moved off Actix handlers.
///
/// Today: `GET /healthz`. Remaining commerce stays on Actix routes until later cutover PRs.
#[must_use]
pub fn commerce_http_kernel() -> AsyncHttpKernel {
    let routes = healthz_matcher();
    AsyncHttpKernel::from_async_fn(move |request: &mut Request| {
        let outcome = routes.apply(request);
        box_future(async move {
            match outcome {
                Ok(found) if found.route_name() == HEALTHZ_ROUTE => Ok(healthz_response()),
                Ok(_) => Err(HttpError::not_found("no handler")),
                Err(error) => Err(error),
            }
        })
    })
}

fn healthz_matcher() -> UrlMatcher {
    let mut collection = RouteCollection::new();
    collection
        .add(Route::with_method(HEALTHZ_ROUTE, "/healthz", Method::Get))
        .expect("healthz route");
    UrlMatcher::new(collection)
}

fn healthz_response() -> Response {
    Response::new(200)
        .with_header("content-type", "application/json")
        .with_body(health_json_body())
}

/// Actix service that forwards to the Serenade kernel (used for `/healthz` during the shell).
#[allow(clippy::future_not_send)]
pub async fn serenade_dispatch(
    request: actix_web::HttpRequest,
    body: actix_web::web::Bytes,
    kernel: actix_web::web::Data<AsyncHttpKernel>,
) -> actix_web::HttpResponse {
    serenade_http_actix::dispatch_async(kernel.get_ref(), &request, body).await
}

/// Registers Serenade-fronted routes on an Actix config (compose with leftover Actix commerce).
pub fn configure_serenade_front(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.route("/healthz", actix_web::web::get().to(serenade_dispatch));
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test as actix_test;
    use actix_web::{web, App};
    use serenade_http::ROUTE_ATTRIBUTE;

    use crate::health::HealthResponse;

    #[actix_web::test]
    async fn healthz_via_serenade_kernel() {
        let kernel = web::Data::new(commerce_http_kernel());
        let app = actix_test::init_service(
            App::new()
                .app_data(kernel)
                .configure(configure_serenade_front),
        )
        .await;
        let req = actix_test::TestRequest::get().uri("/healthz").to_request();
        let resp = actix_test::call_service(&app, req).await;
        assert!(resp.status().is_success());
        let body: HealthResponse = actix_test::read_body_json(resp).await;
        assert_eq!(body.status, "ok");
        assert_eq!(body.kernel, rustashop::kernel_status());
    }

    #[actix_web::test]
    async fn kernel_rejects_unknown_path() {
        let kernel = commerce_http_kernel();
        let response = kernel.handle(Request::new(Method::Get, "/nope")).await;
        assert_eq!(response.status(), 404);
    }

    #[test]
    fn matcher_sets_route_attribute() {
        let matcher = healthz_matcher();
        let mut request = Request::new(Method::Get, "/healthz");
        let found = matcher.apply(&mut request).expect("match");
        assert_eq!(found.route_name(), HEALTHZ_ROUTE);
        assert_eq!(
            request
                .attributes()
                .get::<String>(ROUTE_ATTRIBUTE)
                .map(String::as_str),
            Some(HEALTHZ_ROUTE)
        );
    }
}
