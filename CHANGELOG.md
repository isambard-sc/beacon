# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.1.0] - 2026-10-21

### Added

- `beacon auth` logs in to Isambard Keycloak as the `k8s-gateway` client with the OAuth 2.0 device flow.
- `beacon token` prints the cached access token for the Kubernetes gateway, or tells you to run `beacon auth` if it has expired.
- `beacon whoami` shows the token's claims, and `beacon logout` deletes the cached token.
- An example `examples/opencode/opencode.json` for using a gateway-hosted model with opencode.

[0.1.0]: https://github.com/isambard-sc/beacon/releases/tag/0.1.0
