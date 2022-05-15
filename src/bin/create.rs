use std::sync::Arc;

use reqwest::{
    cookie::{CookieStore, Jar},
    Url,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Filters {
    #[serde(rename = "searchKeywords")]
    search_keywords: String,
}

#[derive(Serialize, Deserialize)]
struct Variables {
    #[serde(rename = "categorySlug")]
    category_slug: String,
    filters: Filters,
    limit: u32,
    skip: u32,
}

#[derive(Serialize, Deserialize)]
struct Question {
    title: String,
    #[serde(rename = "titleSlug")]
    title_slug: String,
}

#[derive(Serialize, Deserialize)]
struct QuestionList {
    questions: Vec<Question>,
}

#[derive(Serialize, Deserialize)]
struct SearchResponseData {
    #[serde(rename = "problemsetQuestionList")]
    question_list: QuestionList,
}

#[derive(Serialize, Deserialize)]
struct SearchResponse {
    data: SearchResponseData,
}

#[derive(Serialize, Deserialize)]

struct SearchQuery {
    query: &'static str,
    variables: Variables,
}

const GRAPHQL_ENDPOINT: &str = "https://leetcode.com/graphql/";
const QUERY: &str = r"query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) {
problemsetQuestionList: questionList(
categorySlug: $categorySlug
limit: $limit
skip: $skip
filters: $filters
) {
total: totalNum
questions: data {
  acRate
  difficulty
  freqBar
  frontendQuestionId: questionFrontendId
  isFavor
  paidOnly: isPaidOnly
  status
  title
  titleSlug
  topicTags {
	name
	id
	slug
  }
  hasSolution
  hasVideoSolution
}
}
}
";

impl SearchQuery {
    fn new(keyword: &str) -> Self {
        Self {
            query: QUERY,
            variables: Variables {
                category_slug: String::new(),
                filters: Filters {
                    search_keywords: keyword.to_string(),
                },
                limit: 3,
                skip: 0,
            },
        }
    }
}

struct LeetcodeScaper {
    cookie: Arc<Jar>,
    client: reqwest::Client,
}

impl LeetcodeScaper {
    fn new() -> Self {
        let cookie = Arc::new(Jar::default());
        let client = reqwest::ClientBuilder::new()
            .cookie_provider(cookie.clone())
            .redirect(reqwest::redirect::Policy::limited(10))
            .user_agent("my-algorithm-lab")
            .build()
            .unwrap();
        Self { client, cookie }
    }

    async fn csrf_token(&self, id: i32) -> Option<(Url, String)> {
        let url =
            Url::parse(format!("https://leetcode.com/problemset/all/?search={}", id).as_str())
                .unwrap();
        self.client.get(url.clone()).send().await.unwrap();
        let value = self.cookie.cookies(&url).unwrap();
        let value = value.to_str().unwrap();
        for segment in value.split(";") {
            let mut segments = segment.split("=");
            let key = segments.next().unwrap().trim();
            let value = segments.next().unwrap().trim();
            if key == "csrftoken" {
                return Some((url, value.to_string()));
            }
        }
        None
    }

    async fn search_questioin(&self, id: i32) -> Vec<Question> {
        let (url, token) = self.csrf_token(id).await.unwrap();
        let id = id.to_string();
        let request = SearchQuery::new(&id);
        let response = self
            .client
            .post(GRAPHQL_ENDPOINT)
            .json(&request)
            .header("Referer", url.as_str())
            .header("x-csrftoken", token)
            .send()
            .await
            .unwrap()
            .json::<SearchResponse>()
            .await
            .unwrap();
        response.data.question_list.questions
    }
}

#[tokio::main]
async fn main() {
    let scaper = LeetcodeScaper::new();
    let id = std::env::args().nth(1).unwrap().parse().unwrap();
    for question in scaper.search_questioin(id).await {
        println!("title: {}, slug: {}", question.title, question.title_slug);
    }
}
