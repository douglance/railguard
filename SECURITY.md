# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| latest  | :white_check_mark: |

Only the latest release receives security updates. We recommend always running the most recent version.

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security issue, please report it responsibly.

### How to Report

1. **Do NOT open a public GitHub issue** for security vulnerabilities
2. Use [GitHub Security Advisories](https://github.com/douglance/railgun/security/advisories/new) to report privately
3. Alternatively, email security concerns to the maintainers directly

### What to Include

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### Response Timeline

- **48 hours**: Initial acknowledgment of your report
- **7 days**: Assessment and triage completed
- **30 days**: Target for patch release (severity dependent)

### What to Expect

- We will acknowledge receipt of your report within 48 hours
- We will provide regular updates on our progress
- We will credit you in the security advisory (unless you prefer to remain anonymous)
- We will notify you when the vulnerability is fixed

## Security Best Practices

When using Railgun:

1. **Keep Railgun updated** - Always use the latest version
2. **Review your policy file** - Ensure `railgun.toml` matches your security requirements
3. **Use strict mode** - Enable `mode = "strict"` for production environments
4. **Audit tool permissions** - Regularly review `[tools]` and `[mcp]` sections
5. **Protect secrets** - Never commit API keys or credentials, even in policy files
