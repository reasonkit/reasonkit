# Security Policy

## Supported Versions

We adhere to Semantic Versioning 2.0.0. Security updates are provided for the current major version.

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

**Do not open a public GitHub issue for security vulnerabilities.**

If you discover a security vulnerability in ReasonKit Web, please report it privately:

1. **Email:** <security@reasonkit.sh>
2. **Response Time:** We are committed to responding to security reports within 48 hours.
3. **Process:**
   - We will investigate and verify the issue.
   - We will develop a patch.
   - We will release a security advisory and a patched version.
   - We will acknowledge your contribution (with permission).

## Responsible Disclosure

We ask that you:

- Give us reasonable time to fix the issue before making it public.
- Do not exploit the vulnerability to view data, modify data, or disrupt service.
- Do not attack our users or infrastructure.

## Security Considerations for Browser Automation

### Browser Sandbox

- **Chromium Isolation:** This package uses `chromiumoxide` to control Chromium/Chrome browsers. The browser runs in a separate process with its own sandbox.
- **Headless Mode:** By default, browsers run headless. The sandbox remains active in headless mode.
- **User Data:** Browser sessions can access cookies, local storage, and credentials. Isolate sessions appropriately.

### MCP Server Security

- **Transport:** The MCP sidecar server uses stdio or HTTP transport. For HTTP, ensure you bind to localhost only in development.
- **Authentication:** When exposing MCP endpoints, implement authentication at the network layer.
- **Input Validation:** All MCP tool inputs are validated before execution to prevent command injection.
- **Timeouts:** Page load and script execution timeouts are enforced to prevent resource exhaustion.

### Network Security

- **TLS Verification:** All HTTPS connections verify certificates by default.
- **Content Capture:** Screenshots and page content may contain sensitive information. Handle captured data appropriately.
- **Cross-Origin:** Be aware of CORS implications when automating web interactions.

### Credential Handling

- **Never log credentials:** The package does not log form inputs or credentials, but custom scripts might.
- **Session isolation:** Use separate browser contexts for different credential scopes.
- **Clear state:** Use incognito/private mode for sensitive operations.

### Memory Safety

- This crate uses `#![forbid(unsafe_code)]` - no unsafe Rust code is present.
- All dependencies are audited via `cargo-audit` in CI.

## Security Audit

This project has undergone internal security audits. However, users should conduct their own security assessment before deploying in sensitive environments.

## Dependency Security

We use `cargo-deny` to ensure:

- No dependencies with known vulnerabilities (RUSTSEC advisories)
- No GPL-licensed dependencies (Apache 2.0 compatibility)
- No yanked crate versions
- Pinned dependency versions via `Cargo.lock`

## Best Practices for Users

1. **Isolate browser sessions:** Use separate browser contexts for different tasks.
2. **Limit permissions:** Run browser automation with minimal system permissions.
3. **Audit scripts:** Review any JavaScript executed in pages for security implications.
4. **Secure MCP transport:** Use authentication when exposing MCP endpoints externally.
5. **Handle captured data:** Treat screenshots and page content as potentially sensitive.
6. **Update regularly:** Keep ReasonKit-web and Chromium/Chrome updated for security patches.
7. **Monitor resource usage:** Browser automation can be resource-intensive; implement limits.

## Chromium Security Notes

This package relies on the Chromium browser's security model:

- **Process isolation:** Each tab runs in a separate renderer process.
- **Site isolation:** Cross-origin iframes are isolated.
- **Sandbox:** Renderer processes run in a restricted sandbox.

For production deployments, ensure you're using an up-to-date, security-patched Chromium/Chrome installation.
