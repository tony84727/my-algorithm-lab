use scraper::Selector;

struct LeetcodeScaper {
    client: reqwest::Client,
}

impl LeetcodeScaper {
    fn new() -> Self {
        let client = reqwest::ClientBuilder::new()
            .cookie_store(true)
            .redirect(reqwest::redirect::Policy::limited(10))
            .user_agent("my-algorithm-lab")
            .build()
            .unwrap();
        Self { client }
    }

    async fn fetch_page(&self, id: i32) {
        let page = self
            .client
            .get(format!(
                "https://leetcode.com/problemset/all/?search={}&page=1",
                id
            ))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let html = scraper::Html::parse_document(&page);
        let selector = Selector::parse("a").unwrap();
        let select = html.select(&selector);
        for el in select.into_iter() {
            if let Some(href) = el.value().attr("href") {
                if href.starts_with("/problems/") {
                    println!("{}", href);
                    for text in el.text() {
                        println!("\t{}", text);
                    }
                    // if let Some(text) = el.text().next() {
                    //     println!("{}", text);
                    // }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let scaper = LeetcodeScaper::new();
    scaper.fetch_page(123).await
}
