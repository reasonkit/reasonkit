//! Browser module tests
//!
//! These tests verify the browser configuration, capture, and navigation types.
//! Note: Full browser integration tests require a running Chrome/Chromium instance.

use reasonkit_web::browser::{
    BrowserConfig, CaptureFormat, CaptureOptions, CaptureResult, NavigationOptions,
    NavigationResult, WaitUntil,
};

#[test]
fn test_browser_config_default() {
    let config = BrowserConfig::default();
    assert!(config.headless);
    assert_eq!(config.width, 1920);
    assert_eq!(config.height, 1080);
    assert!(config.sandbox);
    assert!(config.stealth);
    assert_eq!(config.timeout_ms, 30000);
    assert!(config.user_agent.is_none());
    assert!(config.chrome_path.is_none());
    assert!(config.extra_args.is_empty());
}

#[test]
fn test_browser_config_builder() {
    let config = BrowserConfig::builder()
        .headless(false)
        .viewport(1280, 720)
        .sandbox(false)
        .user_agent("TestBot/1.0")
        .timeout_ms(60000)
        .stealth(false)
        .arg("--disable-gpu")
        .arg("--no-first-run")
        .build();

    assert!(!config.headless);
    assert_eq!(config.width, 1280);
    assert_eq!(config.height, 720);
    assert!(!config.sandbox);
    assert_eq!(config.user_agent, Some("TestBot/1.0".to_string()));
    assert_eq!(config.timeout_ms, 60000);
    assert!(!config.stealth);
    assert_eq!(config.extra_args.len(), 2);
}

#[test]
fn test_capture_format_default() {
    let format = CaptureFormat::default();
    assert_eq!(format, CaptureFormat::Png);
}

#[test]
fn test_capture_format_variants() {
    assert_ne!(CaptureFormat::Png, CaptureFormat::Jpeg);
    assert_ne!(CaptureFormat::Pdf, CaptureFormat::Mhtml);
}

#[test]
fn test_capture_format_serialization() {
    let formats = [
        (CaptureFormat::Png, "\"png\""),
        (CaptureFormat::Jpeg, "\"jpeg\""),
        (CaptureFormat::Webp, "\"webp\""),
        (CaptureFormat::Pdf, "\"pdf\""),
        (CaptureFormat::Mhtml, "\"mhtml\""),
        (CaptureFormat::Html, "\"html\""),
    ];

    for (format, expected) in formats {
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, expected);
    }
}

#[test]
fn test_capture_options_default() {
    let opts = CaptureOptions::default();
    assert_eq!(opts.format, CaptureFormat::Png);
    assert_eq!(opts.quality, 85);
    assert!(opts.full_page);
    assert!(!opts.as_base64);
    assert!(opts.width.is_none());
    assert!(opts.height.is_none());
    assert!(opts.clip_selector.is_none());
}

#[test]
fn test_capture_options_factories() {
    let png = CaptureOptions::png();
    assert_eq!(png.format, CaptureFormat::Png);

    let jpeg = CaptureOptions::jpeg(90);
    assert_eq!(jpeg.format, CaptureFormat::Jpeg);
    assert_eq!(jpeg.quality, 90);

    let pdf = CaptureOptions::pdf();
    assert_eq!(pdf.format, CaptureFormat::Pdf);

    let mhtml = CaptureOptions::mhtml();
    assert_eq!(mhtml.format, CaptureFormat::Mhtml);

    let html = CaptureOptions::html();
    assert_eq!(html.format, CaptureFormat::Html);
}

#[test]
fn test_capture_options_serialization() {
    let opts = CaptureOptions {
        format: CaptureFormat::Jpeg,
        quality: 75,
        full_page: false,
        width: Some(1024),
        height: Some(768),
        clip_selector: Some("#content".to_string()),
        as_base64: true,
    };

    let json = serde_json::to_string(&opts).unwrap();
    assert!(json.contains("\"format\":\"jpeg\""));
    assert!(json.contains("\"quality\":75"));
    assert!(json.contains("\"full_page\":false"));
    assert!(json.contains("\"as_base64\":true"));

    // Deserialize back
    let parsed: CaptureOptions = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.format, CaptureFormat::Jpeg);
    assert_eq!(parsed.quality, 75);
}

#[test]
fn test_capture_result_methods() {
    let result = CaptureResult {
        data: b"hello world".to_vec(),
        format: CaptureFormat::Png,
        base64: None,
        width: Some(100),
        height: Some(50),
        size: 11,
    };

    assert_eq!(result.mime_type(), "image/png");
    assert_eq!(result.extension(), "png");
    assert_eq!(result.to_base64(), "aGVsbG8gd29ybGQ=");
}

#[test]
fn test_capture_result_mime_types() {
    let formats = [
        (CaptureFormat::Png, "image/png", "png"),
        (CaptureFormat::Jpeg, "image/jpeg", "jpg"),
        (CaptureFormat::Webp, "image/webp", "webp"),
        (CaptureFormat::Pdf, "application/pdf", "pdf"),
        (CaptureFormat::Mhtml, "multipart/related", "mhtml"),
        (CaptureFormat::Html, "text/html", "html"),
    ];

    for (format, mime, ext) in formats {
        let result = CaptureResult {
            data: vec![],
            format,
            base64: None,
            width: None,
            height: None,
            size: 0,
        };
        assert_eq!(result.mime_type(), mime);
        assert_eq!(result.extension(), ext);
    }
}

#[test]
fn test_navigation_options_default() {
    let opts = NavigationOptions::default();
    assert_eq!(opts.timeout_ms, 30000);
    assert_eq!(opts.wait_until, WaitUntil::NetworkIdle0);
    assert_eq!(opts.retries, 3);
    assert_eq!(opts.retry_delay_ms, 1000);
    assert!(opts.human_like);
}

#[test]
fn test_wait_until_variants() {
    assert_ne!(WaitUntil::Load, WaitUntil::DomContentLoaded);
    assert_ne!(WaitUntil::NetworkIdle0, WaitUntil::NetworkIdle2);
    assert_eq!(WaitUntil::Load, WaitUntil::Load);
}

#[test]
fn test_navigation_result_structure() {
    let result = NavigationResult {
        final_url: "https://example.com/redirected".to_string(),
        status: Some(200),
        title: Some("Example Page".to_string()),
        duration_ms: 1500,
    };

    assert_eq!(result.final_url, "https://example.com/redirected");
    assert_eq!(result.status, Some(200));
    assert_eq!(result.title, Some("Example Page".to_string()));
    assert_eq!(result.duration_ms, 1500);
}
