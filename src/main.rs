
#[tokio::main]
async fn main() {
    use std::fs::File;
    use std::io::Write;

    println!("First comment: {:?}", get_comments("1").await);

    let mut data = Vec::new();
    
    for i in 1 ..= 6 {
        data.extend(get_comments(&i.to_string()).await);
    }

    let mut file = File::create("dump.json").unwrap();
    write!(file, "{:?}", data).unwrap();
}

async fn get_comments(page: &str) -> Vec<String> {
    use serde_json::Value;

    let client = reqwest::Client::new();
    let resp = client.get("https://api.m.jd.com/")
    .query(&[
        ("appid", "item-v3"),
        ("functionId", "pc_club_productPageComments"),
        ("client", "pc"),
        ("clientVersion", "1.0.0"),
        ("t", "1691465212955"),
        ("loginType", "3"),
        ("uuid", "122270672.16837904800811183431558.1683790480.1690168433.1691460152.8"),
        ("productId", "100002681752"),
        ("score", "0"),
        ("sortType", "5"),
        ("page", page),
        ("pageSize", "10"),
        ("isShadowSku", "0"),
        ("rid", "0"),
        ("fold", "1"),
        ("bbtf", ""),
        ("shield", ""),
    ])  
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/116.0")
    .header("Accept", "application/json, text/javascript, _/_; q=0.01") 
    .header("Accept-Language", "zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2")
    .header("x-referer-page", "https://item.jd.com/100002681752.html")
    .header("x-rp-client", "h5_1.0.0")
    .header("Origin", "https://item.jd.com")
    .header("Connection", "keep-alive")
    .header("Referer", "https://item.jd.com/")
    // 其他请求头
    .send()
    .await
    .unwrap();

    let json: Value = serde_json::from_str(&resp.text().await.unwrap()).unwrap();

    let comments = json["comments"].as_array().unwrap();

    let mut ret = Vec::new();

    for c in comments {
        ret.push(c["content"].as_str().unwrap().into());
    }
    
    ret

}
