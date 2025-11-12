use anyhow::{anyhow, Result};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::Serialize;
use std::time::Duration;

#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
}

pub struct SearchClient {
    client: Client,
}

impl SearchClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .build()
            .unwrap_or_else(|_| Client::new());

        Self { client }
    }

    /// DuckDuckGo文本搜索
    pub async fn search_duckduckgo(&self, keywords: &str, max_results: usize) -> Result<Vec<SearchResult>> {
        let url = format!("https://html.duckduckgo.com/html?q={}", urlencoding::encode(keywords));

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("DDG Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!("DDG Code: {}", response.status()));
        }

        let body = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read response from DDG: {}", e))?;

        self.parse_duckduckgo_html(&body, max_results)
    }

    /// Bing文本搜索
    pub async fn search_bing(&self, keywords: &str, max_results: usize) -> Result<Vec<SearchResult>> {
        let url = format!("https://www.bing.com/search?q={}", urlencoding::encode(keywords));

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Bing请求失败: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!("Bing返回状态码: {}", response.status()));
        }

        let body = response
            .text()
            .await
            .map_err(|e| anyhow!("读取Bing响应失败: {}", e))?;

        self.parse_bing_html(&body, max_results)
    }

    /// 解析DuckDuckGo HTML响应
    fn parse_duckduckgo_html(&self, html: &str, max_results: usize) -> Result<Vec<SearchResult>> {
        let document = Html::parse_document(html);
        let mut results = Vec::new();

        // DuckDuckGo HTML版本的选择器：搜索结果在 div.result 中
        // 或者在 div.links_main 下的 div[class~="result"]
        let selector = match Selector::parse("div.results div[class~='result']") {
            Ok(s) => s,
            Err(_) => Selector::parse("div.results > article").unwrap_or_else(|_| {
                Selector::parse("div[class*='result']").unwrap()
            }),
        };

        for element in document.select(&selector).take(max_results) {
            // 提取标题和链接
            let title_link_selector = Selector::parse("a.result__a").ok();
            let title_link = title_link_selector
                .as_ref()
                .and_then(|sel| element.select(sel).next())
                .and_then(|el| el.value().attr("href"));

            let title = title_link_selector
                .as_ref()
                .and_then(|sel| element.select(sel).next())
                .map(|el| el.inner_html())
                .unwrap_or_default();

            // 提取摘要
            let snippet_selector = Selector::parse("a.result__snippet").ok();
            let snippet = snippet_selector
                .as_ref()
                .and_then(|sel| element.select(sel).next())
                .map(|el| el.inner_html())
                .unwrap_or_default();

            if let Some(url) = title_link {
                let clean_title = clean_html(&title);
                let clean_snippet = clean_html(&snippet);

                if !clean_title.is_empty() && !url.is_empty() {
                    results.push(SearchResult {
                        title: clean_title,
                        url: url.to_string(),
                        snippet: clean_snippet,
                    });
                }
            }
        }

        if results.is_empty() {
            return Err(anyhow!("DuckDuckGo未找到搜索结果"));
        }

        Ok(results)
    }

    /// 解析Bing HTML响应
    fn parse_bing_html(&self, html: &str, max_results: usize) -> Result<Vec<SearchResult>> {
        let document = Html::parse_document(html);
        let mut results = Vec::new();

        // Bing搜索结果选择器：在 li.b_algo 中
        let li_selector = Selector::parse("li.b_algo").unwrap_or_else(|_| {
            Selector::parse("div.b_algoBx").unwrap()
        });

        for element in document.select(&li_selector).take(max_results) {
            // 提取标题和链接
            let h2_selector = Selector::parse("h2 a").ok();
            let (title, url) = h2_selector
                .as_ref()
                .and_then(|sel| element.select(sel).next())
                .map(|el| {
                    let title = el.inner_html();
                    let url = el.value().attr("href").unwrap_or("").to_string();
                    (title, url)
                })
                .unwrap_or_default();

            // 提取摘要
            let p_selector = Selector::parse("p").ok();
            let snippet = p_selector
                .as_ref()
                .and_then(|sel| element.select(sel).next())
                .map(|el| el.inner_html())
                .unwrap_or_default();

            if !title.is_empty() && !url.is_empty() {
                let clean_title = clean_html(&title);
                let clean_snippet = clean_html(&snippet);

                results.push(SearchResult {
                    title: clean_title,
                    url,
                    snippet: clean_snippet,
                });
            }
        }

        if results.is_empty() {
            return Err(anyhow!("Bing未找到搜索结果"));
        }

        Ok(results)
    }
}

/// 清理HTML标签和HTML实体
fn clean_html(html: &str) -> String {
    // 移除HTML标签
    let text = html_escape::decode_html_entities(html);
    
    // 移除HTML注释和脚本
    let re_tags = regex::Regex::new(r"<[^>]+>").unwrap_or_else(|_| {
        regex::Regex::new("").unwrap()
    });
    let cleaned = re_tags.replace_all(&text, "");
    
    // 移除多余空白
    cleaned
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

/// 优雅降级搜索：先尝试DuckDuckGo，失败则尝试Bing
pub async fn search_auto(keywords: &str, max_results: usize) -> Result<Vec<SearchResult>> {
    let client = SearchClient::new();
    
    // 先尝试DuckDuckGo
    match client.search_duckduckgo(keywords, max_results).await {
        Ok(results) => return Ok(results),
        Err(e) => {
            eprintln!("\nDuckDuckGo ERROR: {} \n Try Bing...", e);
        }
    }
    
    // DuckDuckGo失败则尝试Bing
    client.search_bing(keywords, max_results).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_auto() {
        let results = search_auto("Rust programming", 3).await;
        assert!(results.is_ok());
        let results = results.unwrap();
        assert!(!results.is_empty());
    }
}
