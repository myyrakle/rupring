use rupring::NextFunction;

pub fn logger_middleware(
    request: rupring::Request,
    response: rupring::Response,
    next: NextFunction,
) -> rupring::Response {
    println!(
        "Request: {} {}",
        request.method.to_string(),
        request.path.to_string()
    );

    next(request, response)
}
