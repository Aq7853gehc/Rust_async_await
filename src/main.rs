use thiserror::Error;

#[derive(Error,Debug)]
pub enum MyError {
    #[error("Http request failed: {0}")]
    HttpRequest(#[from] reqwest::Error),

    #[error("I/O operation failed {0}")]
    Io(#[from] std::io::Error),
}


#[tokio::main]
async fn main(){
    let res = reqwest::get("https://dummyjson.com/quotes/1").await.unwrap();
    println!("Status: {}",res.status());
    // println!("Header:\n {:#?}",res.headers());
    let body = res.text().await.unwrap();
    println!("Response : {body}");
}



