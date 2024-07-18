#[tokio::main]
async fn main() {
    let resp_result = reqwest::get("http://127.0.0.1:8000/get_no_param").await;
    println!("resp_result {:?}", resp_result);

    let resp = resp_result.expect("Failed to read response text 1 ");
    println!("resp {:?}", resp);

    let resp_text = resp.text().await;
    println!("resp_text {:?}", resp_text);

    // let resp_text_result = resp_text.await;
    // println!("resp_text_await {:?}", resp_text_result);
    //
    let resp_text_v2 = resp_text.expect("Failed to read response text 2 ");
    println!("resp_text_v2 = {:?}", resp_text_v2);

}
