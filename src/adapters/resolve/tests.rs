use super::resolve;

#[test]
fn resolves_web_adapters_and_advertises_them_to_agent_generation() {
    assert_eq!(resolve("nvidia-api").unwrap().id(), "nvidia-api");
    assert_eq!(resolve("ollama").unwrap().id(), "ollama");
    assert_eq!(resolve("ollama-web").unwrap().id(), "ollama-web");
    assert_eq!(resolve("openrouter").unwrap().id(), "openrouter");
    assert!(super::super::AVAILABLE.contains(&"nvidia-api"));
    assert!(super::super::AVAILABLE.contains(&"ollama"));
    assert!(super::super::AVAILABLE.contains(&"ollama-web"));
    assert!(super::super::AVAILABLE.contains(&"openrouter"));
}
