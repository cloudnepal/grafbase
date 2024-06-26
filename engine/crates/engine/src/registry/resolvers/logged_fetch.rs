use runtime::log::LogEvent;
use tracing::{info_span, Instrument};

pub async fn send_logged_request(
    request_id: &str,
    fetch_log_endpoint_url: Option<&str>,
    request_builder: reqwest::RequestBuilder,
) -> Result<reqwest::Response, reqwest::Error> {
    let start_time = web_time::Instant::now();

    let (client, request) = request_builder.build_split();
    let request = request?;

    let origin_url = request.url().clone();
    let method = request.method().to_string();
    let span = info_span!(
        "http_request",
        http.method = %request.method(),
        http.url = &request.url()[..url::Position::AfterQuery]
    );
    let mut response = client.execute(request).instrument(span).await?;

    if let Some(fetch_log_endpoint_url) = fetch_log_endpoint_url {
        let status_code = response.status().as_u16();

        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.split(';').next())
            .map(str::to_owned);

        let (response_to_return, body) = match content_type.as_deref() {
            Some("application/json" | "text/plain" | "text/html") => {
                let mut response_to_return_builder = http::Response::builder().status(status_code);
                response_to_return_builder
                    .headers_mut()
                    .unwrap()
                    .extend(response.headers_mut().drain());
                let bytes = response.bytes().instrument(info_span!("response_data_fetch")).await?;
                let body = String::from_utf8(bytes.to_vec()).ok();
                let response_to_return = response_to_return_builder.body(bytes).expect("must be valid").into();
                (response_to_return, body)
            }
            _ => (response, None),
        };

        let duration = start_time.elapsed();
        let url = format!("{fetch_log_endpoint_url}/log-event");

        reqwest::Client::new()
            .post(&url)
            .json(&LogEvent {
                request_id,
                r#type: common_types::LogEventType::NestedRequest {
                    url: origin_url.to_string(),
                    method,
                    status_code,
                    duration,
                    body,
                    content_type,
                },
            })
            .send()
            .instrument(info_span!("log_event", url = &url))
            .await?;

        Ok(response_to_return)
    } else {
        Ok(response)
    }
}
