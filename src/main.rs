use lambda_runtime::handler_fn;
use log::info;
use serde::{ Deserialize, Serialize };
use sonya::validate;

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;

    Ok(())
}

async fn handler(req: Request, _ctx: lambda_runtime::Context) -> Response {
    info!("handling a request...");
    // let config = aws_config::load_from_env().await;
    let res = validate(&req.body);

    Ok(SuccessResponse {
        body: format!(
            "the lambda has successfully validated the JSON with result:{res}"
        ),
    })
}

#[derive(Deserialize)]
struct Request {
    pub body: String,
}

#[derive(Debug, Serialize)]
struct SuccessResponse {
    pub body: String,
}

#[derive(Debug, Serialize)]
struct FailureResponse {
    pub body: String,
}

impl std::fmt::Display for FailureResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

impl std::error::Error for FailureResponse {}

type Response = Result<SuccessResponse, FailureResponse>;
