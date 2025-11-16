use axum::{middleware::Next, http::{Request, Response, header}, body::Body, http::Method};

/* 
Axum middleware function, which runs for every incoming HTTP request
This middleware adds the necessary HTTP headers to allow cross-origin requests
parameters: 
    req: Request<Body>: the incoming HTTP request
    next: Next: the next handler in the chain (route)
It returns a Response<Body>, which is sent back to the client
*/
pub async fn cors_middleware(req: Request<Body>, next: Next) -> Response<Body> {
    // Browsers send an OPTIONS request before making a real request, called a preflight request
    if req.method() == Method::OPTIONS {
        // Respond directly to preflight requests
        let mut response = Response::new(Body::empty());
        let headers = response.headers_mut();
         // allow any origin
        headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
        // allowed HTTP methods
        headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, "GET,POST,DELETE,OPTIONS".parse().unwrap());
        // allowed headers
        headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, "*".parse().unwrap());
        return response;
    }

    let mut response = next.run(req).await; // passes the request to the actual route handler
    let headers = response.headers_mut();
    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, "GET,POST,DELETE,OPTIONS".parse().unwrap());
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, "*".parse().unwrap());
    response
}