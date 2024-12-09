// Reference: https://docs.aws.amazon.com/ko_kr/lambda/latest/dg/runtimes-api.html
fn get_aws_lambda_runtime_api() -> Option<String> {
    std::env::var("AWS_LAMBDA_RUNTIME_API").ok()
}
