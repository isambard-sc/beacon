# beacon

Rust CLI that logs in to Isambard Keycloak with the OAuth 2.0 device flow (RFC 8628) and prints an access-token JWT for the Kubernetes gateway. User-facing docs are in README.md; releases in RELEASE.md. Ported from [chepstow](https://github.com/isambard-sc/chepstow), with refresh and revocation removed.

**Keep the code minimal.** Prefer the smallest change that works. Don't add crates, abstractions or workarounds for server-side problems (for example token lifetime or routing).

## Layout

```
src/main.rs    clap args, ISSUER/CLIENT_ID/SCOPE consts, subcommands, valid_token() expiry check
src/oidc.rs    discovery and device flow (oauth2 5 BasicClient)
src/cache.rs   token cache: dirs::cache_dir()/beacon/token.json, mode 0600
src/jwt.rs     unverified JWT payload decode
tests/cli.rs   runs the binary against a temp cache
```

## Design choices

- Blocking reqwest + rustls, no async runtime. `default-features = false` on every crate.
- Endpoints come from `{issuer}/.well-known/openid-configuration`. They are never hardcoded.
- Public client `k8s-gateway`: `AuthType::RequestBody`, no secret, scope `openid profile`. The client issues access tokens only, so there is no refresh or revocation. Any refresh token the server sends is ignored.
- The issuer, client ID and scope are hardcoded consts with no flags, env vars or config file, so users aren't invited to point beacon elsewhere. The issuer is Keycloak prod.
- The oauth2 crate handles polling (`authorization_pending`, `slow_down` +5s, `expired_token`, `access_denied`). The sleep function is injected so tests don't wait.
- `expires_at` comes from the access token's JWT `exp` claim, in unix seconds.
- `token` prints only the token on stdout. If the token is missing or has less than 60s left, it prints an error telling the user to run `beacon auth` on stderr and exits 1, and never starts a login. Other human-readable output also goes to stderr.
- `whoami` reads the cache. Missing claims print as `<missing>` so a missing Keycloak mapper is obvious.
- `logout` deletes the cache.

## Rules

- Lints: `unsafe_code` forbidden; `unwrap_used`, `expect_used` and `dbg_macro` denied, in tests too. Tests return `anyhow::Result` and use `?`.
- Before finishing a change, run:
  ```sh
  cargo fmt --check
  cargo clippy --all-targets -- -D warnings
  cargo test
  ```
- Add user-visible changes to the `Unreleased` section of CHANGELOG.md (Keep a Changelog format; CI checks it with kacl).
- CI is in `.github/workflows/` (check, build, release, and actions for actionlint + zizmor), ported from chepstow. Actions are pinned by SHA. Dependabot (`.github/dependabot.yml`) updates cargo weekly and actions monthly.
