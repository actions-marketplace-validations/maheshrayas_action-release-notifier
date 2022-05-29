use anyhow::Result;
use release_notifier::{Input, InputType};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .with_line_number(false)
        .with_target(false)
        .with_file(false)
        .with_max_level(tracing::Level::INFO)
        .init();
    process().await?;
    Ok(())
}

async fn process() -> Result<()> {
    let repo: String = env::var("INPUT_REPO").expect("Missing input parameter: repo");
    let days: i64 = env::var("INPUT_DAYS")
        .unwrap_or_else(|_| 1.to_string())
        .parse::<i64>()
        .unwrap();
    let token = format!(
        "token {}",
        env::var("INPUT_GITHUB_TOKEN").expect("Missing input parameter: github_token")
    );
    let input_type = env::var("INPUT_TYPE").expect("Missing input parameter: type");

    // intialize the struct
    let input = Input::new(input_type, token, repo, days);

    let m = match input.input_type {
        InputType::Github => input.gh().await?,
        InputType::Rss => input.rss().await?,
    };
    Ok(m)
}

#[tokio::test]
async fn test_main() {
    use std::time::Instant;
    let start = Instant::now();
    let gh_token = &env::var("TOKEN").unwrap();
    env::set_var("INPUT_REPO", "https://github.com/maheshrayas/action-release-notifier,https://github.com/maheshrayas/action-pr-comment-delete,https://github.com/kubernetes/kubernetes");
    env::set_var("INPUT_DAYS", "4");
    env::set_var("GITHUB_REPOSITORY", "maheshrayas/action-release-notifier");

    env::set_var("INPUT_GITHUB_TOKEN", gh_token);
    if let Err(_) = process().await {
        panic!("Failed",);
    }
    let duration = start.elapsed();
    println!("Time taken for execution is: {:?}", duration);
}
