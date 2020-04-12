use interactive_paper::{resolve};

#[tokio::main]
async fn main() {
    let input = "go to the previous question section 2";

    let res = resolve(input).await;

    println!("Response {:#?}",res);
}
