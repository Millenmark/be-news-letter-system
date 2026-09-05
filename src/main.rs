use be_email_news_letter::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run("127.0.0.1:0")?.await
}
