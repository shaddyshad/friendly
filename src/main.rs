use interactive_paper::{resolve};

#[tokio::main]
async fn main() {
    let input = "go to question 3 section 2";

    let res = resolve(input).await;

    println!("Response {:#?}",res);
}
