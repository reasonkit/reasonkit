//! Extraction module tests
//!
//! These tests verify the content, link, and metadata extraction functionality.

use reasonkit_web::extraction::{
    ExtractedContent, ExtractedLink, LinkType, OpenGraphData, PageMetadata, TwitterCardData,
};

#[test]
fn test_extracted_content_structure() {
    let content = ExtractedContent {
        text: "Hello world. This is a test.".to_string(),
        markdown: Some("Hello world. This is a test.".to_string()),
        html: "<p>Hello world. This is a test.</p>".to_string(),
        word_count: 6,
        char_count: 28,
        from_main: true,
    };

    assert_eq!(content.word_count, 6);
    assert_eq!(content.char_count, 28);
    assert!(content.from_main);
    assert!(content.markdown.is_some());
}

#[test]
fn test_link_type_variants() {
    assert_ne!(LinkType::Internal, LinkType::External);
    assert_eq!(LinkType::Internal, LinkType::Internal);

    // Serialization
    let json = serde_json::to_string(&LinkType::External).unwrap();
    assert_eq!(json, "\"external\"");

    let json = serde_json::to_string(&LinkType::Email).unwrap();
    assert_eq!(json, "\"email\"");
}

#[test]
fn test_extracted_link_structure() {
    let link = ExtractedLink {
        url: "https://example.com/page".to_string(),
        text: "Example Page".to_string(),
        title: Some("Visit Example".to_string()),
        link_type: LinkType::External,
        rel: Some("nofollow".to_string()),
        new_tab: true,
        context: Some("Check out this Example Page for more info".to_string()),
        position: 5,
    };

    assert_eq!(link.url, "https://example.com/page");
    assert_eq!(link.link_type, LinkType::External);
    assert!(link.new_tab);
    assert_eq!(link.position, 5);
}

#[test]
fn test_extracted_link_serialization() {
    let link = ExtractedLink {
        url: "https://test.com".to_string(),
        text: "Test".to_string(),
        title: None,
        link_type: LinkType::Internal,
        rel: None,
        new_tab: false,
        context: None,
        position: 0,
    };

    let json = serde_json::to_string(&link).unwrap();
    assert!(json.contains("\"url\":\"https://test.com\""));
    assert!(json.contains("\"link_type\":\"internal\""));
}

#[test]
fn test_page_metadata_default() {
    let meta = PageMetadata::default();
    assert!(meta.title.is_none());
    assert!(meta.description.is_none());
    assert!(meta.keywords.is_empty());
    assert!(meta.json_ld.is_empty());
    assert!(meta.meta_tags.is_empty());
}

#[test]
fn test_page_metadata_full() {
    let meta = PageMetadata {
        title: Some("Test Page".to_string()),
        description: Some("A test page description".to_string()),
        canonical: Some("https://example.com/test".to_string()),
        language: Some("en".to_string()),
        author: Some("Test Author".to_string()),
        keywords: vec!["test".to_string(), "example".to_string()],
        open_graph: OpenGraphData {
            title: Some("OG Title".to_string()),
            description: Some("OG Description".to_string()),
            image: Some("https://example.com/image.jpg".to_string()),
            url: Some("https://example.com/test".to_string()),
            og_type: Some("article".to_string()),
            site_name: Some("Example Site".to_string()),
            locale: Some("en_US".to_string()),
        },
        twitter_card: TwitterCardData {
            card: Some("summary_large_image".to_string()),
            title: Some("Twitter Title".to_string()),
            description: Some("Twitter Description".to_string()),
            image: Some("https://example.com/twitter.jpg".to_string()),
            site: Some("@example".to_string()),
            creator: Some("@author".to_string()),
        },
        favicon: Some("/favicon.ico".to_string()),
        meta_tags: [
            ("viewport".to_string(), "width=device-width".to_string()),
            ("robots".to_string(), "index, follow".to_string()),
        ]
        .into_iter()
        .collect(),
        json_ld: vec![serde_json::json!({
            "@type": "Article",
            "headline": "Test"
        })],
    };

    assert_eq!(meta.title, Some("Test Page".to_string()));
    assert_eq!(meta.keywords.len(), 2);
    assert_eq!(meta.open_graph.og_type, Some("article".to_string()));
    assert_eq!(
        meta.twitter_card.card,
        Some("summary_large_image".to_string())
    );
    assert_eq!(meta.meta_tags.len(), 2);
    assert_eq!(meta.json_ld.len(), 1);
}

#[test]
fn test_open_graph_data() {
    let og = OpenGraphData {
        title: Some("Title".to_string()),
        description: Some("Description".to_string()),
        image: Some("https://example.com/img.jpg".to_string()),
        url: Some("https://example.com".to_string()),
        og_type: Some("website".to_string()),
        site_name: Some("Example".to_string()),
        locale: Some("en_US".to_string()),
    };

    let json = serde_json::to_string(&og).unwrap();
    assert!(json.contains("\"title\":\"Title\""));
    assert!(json.contains("\"og_type\":\"website\""));
}

#[test]
fn test_twitter_card_data() {
    let tw = TwitterCardData {
        card: Some("summary".to_string()),
        title: Some("Title".to_string()),
        description: Some("Description".to_string()),
        image: Some("https://example.com/img.jpg".to_string()),
        site: Some("@site".to_string()),
        creator: Some("@creator".to_string()),
    };

    let json = serde_json::to_string(&tw).unwrap();
    assert!(json.contains("\"card\":\"summary\""));
    assert!(json.contains("\"site\":\"@site\""));
}

#[test]
fn test_page_metadata_serialization() {
    let meta = PageMetadata {
        title: Some("Test".to_string()),
        description: Some("Description".to_string()),
        ..Default::default()
    };

    let json = serde_json::to_string(&meta).unwrap();
    assert!(json.contains("\"title\":\"Test\""));

    // Deserialize back
    let parsed: PageMetadata = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.title, Some("Test".to_string()));
}
