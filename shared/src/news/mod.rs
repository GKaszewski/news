use anyhow::Result;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, SqlitePool};

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct NewsItem {
    pub title: String,
    pub author: Option<String>,
    pub body: String,
    pub url: Option<String>,
}

impl Default for NewsItem {
    fn default() -> Self {
        NewsItem {
            title: "".to_string(),
            author: None,
            body: "".to_string(),
            url: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NewsSource {
    BBC,
    Guardian,
}

// This function will parse the HTML content of a BBC news article
// and return a NewsItem struct with the title, author, and body of the article.
fn parse_bbc_news(file_content: &str, url: &str) -> Result<NewsItem> {
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

    let author = document
        .select(&author_selector)
        .next()
        .map(|author| author.text().collect::<String>());

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
    news.url = Some(url.to_string());

    Ok(news)
}

// This function will parse the HTML content of a Guardian news article
// and return a NewsItem struct with the title, author, and body of the article.
fn parse_guardian_news(file_content: &str, url: &str) -> Result<NewsItem> {
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
    news.url = Some(url.to_string());

    Ok(news)
}

async fn fetch_html(url: &str) -> Result<String> {
    let client = reqwest::Client::builder()
        .user_agent("insomnia/8.6.1")
        .build()?;

    let html = client.get(url).send().await?.text().await?;
    Ok(html)
}

pub async fn fetch_news_from_url(
    db: &SqlitePool,
    url: &str,
    source: NewsSource,
) -> Result<NewsItem> {
    let query = sqlx::query_as::<_, NewsItem>(
        "SELECT title, author, body, url FROM news_items WHERE url = ?1",
    );
    let news_item: Vec<NewsItem> = query.fetch_all(db).await?;

    if !news_item.is_empty() {
        return Ok(news_item[0].clone());
    }

    let content = fetch_html(url).await?;
    match source {
        NewsSource::BBC => parse_bbc_news(&content, url),
        NewsSource::Guardian => parse_guardian_news(&content, url),
    }
}
