use reqwest::Client;


#[derive(Debug, Deserialize)]
struct Address {
    address1: String,
    address2: String,
    address3: String,
    prefcode: String,
    zipcode: String,
}

#[derive(Debug, Deserialize)]
struct ZipCloudResponse {
    status: u32,
    results: Vec<Address>,
}

async fn get_download_contents() {

    let client = Client::new();
    let url = "https://zipcloud.ibsnet.co.jp/api/search";
    let response = client
        .get(url)
        .query(&[("zipcode", "1000002")])
        .send()
        .await;

    let thread_one = std::thread::spawn(|| response.json::<ZipCloudResponse>().await?);
    let thread_second = std::thread::spawn(|| download("https://www.bar.com"));

    thread_one.join().expect("thread one panicked");
    thread_second.join().expect("thread one panicked");    
}

fn main() {
    let future = get_download_contents();
    futures::executor::block_on(future);
}

