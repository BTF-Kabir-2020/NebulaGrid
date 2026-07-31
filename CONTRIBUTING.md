# Contributing to NebulaGrid

## Development Setup

See [docs/setup.md](docs/setup.md) for local development setup.

## Code Style

- **Rust**: `cargo fmt` + `cargo clippy --all -- -D warnings`
- **TypeScript/React**: ESLint + Prettier (`npm run lint`, `npm run format`)
- No unnecessary comments in source code

## Pull Request Process

1. Ensure all CI checks pass (Rust tests + Clippy + Frontend build + lint)
2. Update docs/api.md if adding or changing API endpoints
3. Update MAPS.md if adding or renaming files
4. Update docs when behavior or APIs change

## Commit Messages

Use conventional commits: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`

## Project Structure

- `control-plane/` — Rust workspace (gateway + microservice crates)
- `dashboard/` — React + TypeScript frontend
- `agent/` — Rust agent binary
- `labs/` — Lab environment with Docker Compose
- `docs/` — Documentation
- `deployment/` — Production Docker/K8s/Terraform configs
