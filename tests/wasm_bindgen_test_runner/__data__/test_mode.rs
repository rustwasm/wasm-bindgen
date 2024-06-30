enum TestMode {
    Default,
    Deno,
    Node,
    BrowserDefault,
    BrowserChrome,
    BrowserEdge,
    BrowserFirefox,
    #[cfg(host_os = "macos")]
    BrowserSafari,
}
