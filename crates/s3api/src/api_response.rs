pub fn write_success_response(response: Vec<u8>) -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .insert_header(("Content-Length", response.len()))
        .body(response)
}

pub fn write_error_response(
    req: &actix_web::HttpRequest,
    error: s3err::ApiErrorCode,
) -> actix_web::HttpResponse {
    let mut w = actix_web::HttpResponseBuilder::new(error.status_code());
    let (builder, handled) = write_error_response_headers(&mut w, error);
    if !handled {
        write_error_response_no_headers(builder, error, req.path())
    } else {
        w.into()
    }
}

pub fn write_error_response_headers(
    w: &mut actix_web::HttpResponseBuilder,
    error: s3err::ApiErrorCode,
) -> (&mut actix_web::HttpResponseBuilder, bool) {
    let status = error.status_code();
    (w.status(status), false)

    // TODO: website routing rules unhandled for now
}

pub fn write_error_response_no_headers(
    w: &mut actix_web::HttpResponseBuilder,
    error: s3err::ApiErrorCode,
    resource: &str,
) -> actix_web::HttpResponse {
    let status = error.status_code();
    let error_message = error.description();
    let error_response = format!(
        "<Error><Code>{}</Code><Message>{}</Message><Resource>{}</Resource></Error>",
        status.as_str(),
        error_message,
        resource
    );
    w.body(error_response)
}
