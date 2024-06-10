use anyhow::{Ok, Result};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsItem {
    pub title: String,
    pub author: Option<String>,
    pub body: String,
}

impl Default for NewsItem {
    fn default() -> Self {
        NewsItem {
            title: "".to_string(),
            author: None,
            body: "".to_string(),
        }
    }
}

pub enum NewsSource {
    BBC,
    Guardian,
}

// This function will parse the HTML content of a BBC news article
// and return a NewsItem struct with the title, author, and body of the article.
fn parse_bbc_news(file_content: &str) -> Result<NewsItem> {
    let mut news = NewsItem::default();
    let document = Html::parse_document(file_content);

    let title_selector = Selector::parse(r"[data-component='headline-block']").unwrap();
    let author_selector = Selector::parse(r"[data-component='byline-block']").unwrap();
    let body_selector = Selector::parse(r"[data-component='text-block']").unwrap();
    let p_tags_selector = Selector::parse("p").unwrap();

    let title = document
        .select(&title_selector)
        .next()
        .ok_or(anyhow::anyhow!("Failed to find title"))?;

    let author = match document.select(&author_selector).next() {
        Some(author) => Some(author.text().collect::<String>()),
        None => None,
    };

    let body = document
        .select(&body_selector)
        .next()
        .ok_or(anyhow::anyhow!("Failed to find body"))?;

    let p_tags = body.select(&p_tags_selector);

    let mut body_text = String::new();
    for p_tag in p_tags {
        body_text.push_str(&p_tag.text().collect::<String>());
    }

    news.title = title.text().collect::<String>();
    news.author = author;
    news.body = body_text;

    Ok(news)
}

// This function will parse the HTML content of a Guardian news article
// and return a NewsItem struct with the title, author, and body of the article.
fn parse_guardian_news(file_content: &str) -> Result<NewsItem> {
    let mut news = NewsItem::default();
    let document = Html::parse_document(file_content);

    let h1_selector = Selector::parse(r"h1").unwrap();
    let p_tags_selector = Selector::parse("p").unwrap();

    let title_selector = Selector::parse(r"[data-gu-name='headline']").unwrap();
    let body_selector = Selector::parse(r"[data-gu-name='body']").unwrap();

    let title = document
        .select(&title_selector)
        .next()
        .ok_or(anyhow::anyhow!("Failed to find title element"))?
        .select(&h1_selector)
        .next()
        .ok_or(anyhow::anyhow!("Failed to find title h1"))?;

    let body = document
        .select(&body_selector)
        .next()
        .ok_or(anyhow::anyhow!("Failed to find body element"))?;

    let p_tags = body.select(&p_tags_selector);
    let mut body_text = String::new();
    for p_tag in p_tags {
        body_text.push_str(&p_tag.text().collect::<String>());
    }

    news.title = title.text().collect::<String>();
    news.body = body_text;

    Ok(news)
}

async fn fetch_html(url: &str) -> Result<String> {
    let client = reqwest::Client::builder()
        .user_agent("insomnia/8.6.1")
        .build()?;

    let html = client.get(url).send().await?.text().await?;
    Ok(html)
}

pub async fn fetch_news_from_url(url: &str, source: NewsSource) -> Result<NewsItem> {
    let content = fetch_html(url).await?;
    match source {
        NewsSource::BBC => parse_bbc_news(&content),
        NewsSource::Guardian => parse_guardian_news(&content),
    }
}
