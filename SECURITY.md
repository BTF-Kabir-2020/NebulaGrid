# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | ✅ Active development |

## Reporting a Vulnerability

Please report security vulnerabilities by opening an issue at:
https://github.com/your-org/nebula-grid/issues

Do not disclose security vulnerabilities publicly until they have been addressed by the maintainers.

## Authentication

NebulaGrid uses:
- **JWT** for API authentication with 24h token expiry
- **Argon2id** for password hashing
- **Refresh tokens** for session management
- **API tokens** (`ng_*` prefixed) for programmatic access

**Never** commit `.env` files or hardcode credentials.
