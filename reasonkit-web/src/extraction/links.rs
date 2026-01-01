//! Link extraction
//!
//! This module extracts all links from web pages with context and metadata.

use crate::browser::PageHandle;
use crate::error::{ExtractionError, Result};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument};

/// Type of link
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LinkType {
    /// Internal link (same domain)
    Internal,
    /// External link (different domain)
    External,
    /// Anchor link (same page)
    Anchor,
    /// mailto: link
    Email,
    /// tel: link
    Phone,
    /// JavaScript link
    JavaScript,
    /// Other/unknown
    Other,
}

/// An extracted link with context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedLink {
    /// The href URL
    pub url: String,
    /// Link text content
    pub text: String,
    /// Title attribute
    pub title: Option<String>,
    /// Type of link
    pub link_type: LinkType,
    /// Rel attribute
    pub rel: Option<String>,
    /// Whether it opens in a new tab
    pub new_tab: bool,
    /// Surrounding context (nearby text)
    pub context: Option<String>,
    /// Position in document (order found)
    pub position: usize,
}

/// Link extraction functionality
pub struct LinkExtractor;

impl LinkExtractor {
    /// Extract all links from the page
    #[instrument(skip(page))]
    pub async fn extract_all(page: &PageHandle) -> Result<Vec<ExtractedLink>> {
        info!("Extracting all links");

        let script = r#"
            (() => {
                const links = [];
                const baseUrl = window.location.origin;
                const currentHost = window.location.hostname;

                document.querySelectorAll('a[href]').forEach((el, index) => {
                    const href = el.getAttribute('href') || '';
                    const text = el.innerText.trim() || el.textContent.trim();
                    const title = el.getAttribute('title');
                    const rel = el.getAttribute('rel');
                    const target = el.getAttribute('target');

                    // Get context (parent text or siblings)
                    let context = '';
                    try {
                        const parent = el.parentElement;
                        if (parent) {
                            context = parent.innerText.substring(0, 200);
                        }
                    } catch (e) {}

                    // Determine link type
                    let linkType = 'other';
                    if (href.startsWith('#')) {
                        linkType = 'anchor';
                    } else if (href.startsWith('mailto:')) {
                        linkType = 'email';
                    } else if (href.startsWith('tel:')) {
                        linkType = 'phone';
                    } else if (href.startsWith('javascript:')) {
                        linkType = 'javascript';
                    } else {
                        try {
                            const url = new URL(href, baseUrl);
                            if (url.hostname === currentHost) {
                                linkType = 'internal';
                            } else {
                                linkType = 'external';
                            }
                        } catch (e) {
                            linkType = 'other';
                        }
                    }

                    // Resolve relative URLs
                    let fullUrl = href;
                    if (!href.startsWith('http') && !href.startsWith('mailto:') &&
                        !href.startsWith('tel:') && !href.startsWith('javascript:') &&
                        !href.startsWith('#')) {
                        try {
                            fullUrl = new URL(href, baseUrl).href;
                        } catch (e) {}
                    }

                    links.push({
                        url: fullUrl,
                        text: text.substring(0, 500),
                        title: title,
                        linkType: linkType,
                        rel: rel,
                        newTab: target === '_blank',
                        context: context,
                        position: index
                    });
                });

                return links;
            })()
        "#;

        let result: Vec<serde_json::Value> = page
            .page
            .evaluate(script)
            .await
            .map_err(|e| ExtractionError::ExtractionFailed(e.to_string()))?
            .into_value()
            .map_err(|e| ExtractionError::ExtractionFailed(e.to_string()))?;

        let links: Vec<ExtractedLink> = result
            .into_iter()
            .map(|v| {
                let link_type_str = v["linkType"].as_str().unwrap_or("other");
                let link_type = match link_type_str {
                    "internal" => LinkType::Internal,
                    "external" => LinkType::External,
                    "anchor" => LinkType::Anchor,
                    "email" => LinkType::Email,
                    "phone" => LinkType::Phone,
                    "javascript" => LinkType::JavaScript,
                    _ => LinkType::Other,
                };

                ExtractedLink {
                    url: v["url"].as_str().unwrap_or("").to_string(),
                    text: v["text"].as_str().unwrap_or("").to_string(),
                    title: v["title"].as_str().map(String::from),
                    link_type,
                    rel: v["rel"].as_str().map(String::from),
                    new_tab: v["newTab"].as_bool().unwrap_or(false),
                    context: v["context"].as_str().map(String::from),
                    position: v["position"].as_u64().unwrap_or(0) as usize,
                }
            })
            .collect();

        debug!("Extracted {} links", links.len());
        Ok(links)
    }

    /// Extract only external links
    #[instrument(skip(page))]
    pub async fn extract_external(page: &PageHandle) -> Result<Vec<ExtractedLink>> {
        let all = Self::extract_all(page).await?;
        Ok(all
            .into_iter()
            .filter(|l| l.link_type == LinkType::External)
            .collect())
    }

    /// Extract only internal links
    #[instrument(skip(page))]
    pub async fn extract_internal(page: &PageHandle) -> Result<Vec<ExtractedLink>> {
        let all = Self::extract_all(page).await?;
        Ok(all
            .into_iter()
            .filter(|l| l.link_type == LinkType::Internal)
            .collect())
    }

    /// Extract links matching a pattern
    #[instrument(skip(page))]
    pub async fn extract_matching(page: &PageHandle, pattern: &str) -> Result<Vec<ExtractedLink>> {
        let all = Self::extract_all(page).await?;
        let regex = regex::Regex::new(pattern)
            .map_err(|e| ExtractionError::InvalidSelector(format!("Invalid regex: {}", e)))?;

        Ok(all.into_iter().filter(|l| regex.is_match(&l.url)).collect())
    }

    /// Extract links from a specific container
    #[instrument(skip(page))]
    pub async fn extract_from_selector(
        page: &PageHandle,
        selector: &str,
    ) -> Result<Vec<ExtractedLink>> {
        let script = format!(
            r#"
            (() => {{
                const container = document.querySelector('{}');
                if (!container) return [];

                const links = [];
                const baseUrl = window.location.origin;
                const currentHost = window.location.hostname;

                container.querySelectorAll('a[href]').forEach((el, index) => {{
                    const href = el.getAttribute('href') || '';
                    const text = el.innerText.trim() || el.textContent.trim();
                    const title = el.getAttribute('title');
                    const rel = el.getAttribute('rel');
                    const target = el.getAttribute('target');

                    let linkType = 'other';
                    if (href.startsWith('#')) {{
                        linkType = 'anchor';
                    }} else if (href.startsWith('mailto:')) {{
                        linkType = 'email';
                    }} else if (href.startsWith('tel:')) {{
                        linkType = 'phone';
                    }} else if (href.startsWith('javascript:')) {{
                        linkType = 'javascript';
                    }} else {{
                        try {{
                            const url = new URL(href, baseUrl);
                            linkType = url.hostname === currentHost ? 'internal' : 'external';
                        }} catch (e) {{}}
                    }}

                    let fullUrl = href;
                    if (!href.startsWith('http') && !href.startsWith('mailto:') &&
                        !href.startsWith('tel:') && !href.startsWith('javascript:') &&
                        !href.startsWith('#')) {{
                        try {{
                            fullUrl = new URL(href, baseUrl).href;
                        }} catch (e) {{}}
                    }}

                    links.push({{
                        url: fullUrl,
                        text: text.substring(0, 500),
                        title: title,
                        linkType: linkType,
                        rel: rel,
                        newTab: target === '_blank',
                        context: null,
                        position: index
                    }});
                }});

                return links;
            }})()
            "#,
            selector.replace('\'', "\\'")
        );

        let result: Vec<serde_json::Value> = page
            .page
            .evaluate(script.as_str())
            .await
            .map_err(|e| ExtractionError::ExtractionFailed(e.to_string()))?
            .into_value()
            .map_err(|e| ExtractionError::ExtractionFailed(e.to_string()))?;

        let links: Vec<ExtractedLink> = result
            .into_iter()
            .map(|v| {
                let link_type_str = v["linkType"].as_str().unwrap_or("other");
                let link_type = match link_type_str {
                    "internal" => LinkType::Internal,
                    "external" => LinkType::External,
                    "anchor" => LinkType::Anchor,
                    "email" => LinkType::Email,
                    "phone" => LinkType::Phone,
                    "javascript" => LinkType::JavaScript,
                    _ => LinkType::Other,
                };

                ExtractedLink {
                    url: v["url"].as_str().unwrap_or("").to_string(),
                    text: v["text"].as_str().unwrap_or("").to_string(),
                    title: v["title"].as_str().map(String::from),
                    link_type,
                    rel: v["rel"].as_str().map(String::from),
                    new_tab: v["newTab"].as_bool().unwrap_or(false),
                    context: None,
                    position: v["position"].as_u64().unwrap_or(0) as usize,
                }
            })
            .collect();

        debug!("Extracted {} links from {}", links.len(), selector);
        Ok(links)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_type_serialization() {
        let lt = LinkType::External;
        let json = serde_json::to_string(&lt).unwrap();
        assert_eq!(json, "\"external\"");
    }

    #[test]
    fn test_extracted_link_structure() {
        let link = ExtractedLink {
            url: "https://example.com".to_string(),
            text: "Example".to_string(),
            title: Some("Example Site".to_string()),
            link_type: LinkType::External,
            rel: Some("nofollow".to_string()),
            new_tab: true,
            context: Some("Click here: Example to visit".to_string()),
            position: 0,
        };

        assert_eq!(link.link_type, LinkType::External);
        assert!(link.new_tab);
        assert!(link.title.is_some());
    }
}
