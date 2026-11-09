# Releasing noms-core

The published library is `noms-core`; its Rust import is `noms_core`. Applications
can alias it to `noms` in their own manifests. Only this package is published.
The CLI and server packages have `publish = false` and retain their current names.

## Local checks

The development toolchain is pinned in `mise.toml` and `mise.lock`, following the
same pinned-action pattern as git-commitfs. Regenerate `mise.lock` only with the
CI-pinned mise version, and update both workflow pins and `min_version` when
changing the lockfile writer. Rust 1.88 is the library minimum;
workspace development dependencies use the newer pinned toolchain.

```sh
mise trust
mise install
mise exec -- cargo fmt --all --check
mise exec -- cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
mise exec -- cargo test --workspace --all-features --locked
mise run check-workflows
mise exec -- scripts/check-core-package
```

For uncommitted local edits, use `scripts/check-core-package --allow-dirty`.
This packages the library, extracts the archive, creates a standalone binary under
ignored `dev/core-consumer.*`, and verifies email and Telegram delivery to local
receivers. It uses the public `noms` dependency alias and has its own workspace and
dependency resolution. It never contacts real notification services. The created
directory is retained for inspection; it can be deleted after checking.

CI runs formatting, strict Clippy, workspace tests and API documentation; checks
core provider features on Linux and the full core on macOS; builds the library
with Rust 1.88; and runs the packaged consumer. It uploads the `.crate` artifact.

## Initial publication

crates.io currently requires the crate to exist before Trusted Publishing can be
configured: the first publication requires a crates.io API token. See the
[official prerequisites](https://crates.io/docs/trusted-publishing).

1. Review and merge this release setup into `main`. Leave the GitHub repository
   variable `PUBLISH_CRATE` unset (or `false`) until initial publication is complete.
2. Commit the intended `0.1.0` release, then create and push its annotated tag:

   ```sh
   git tag -a noms-core-v0.1.0 -m 'noms-core 0.1.0'
   git push origin main noms-core-v0.1.0
   ```

3. Wait for the release workflow's checks and package-consumer verification. With
   publishing disabled, it creates a GitHub release with the verified source archive.
4. From a clean checkout of that exact tag, authenticate locally with a narrowly
   scoped crates.io API token and publish the initial version:

   ```sh
   cargo login
   mise exec -- cargo publish -p noms-core --locked
   ```

   Supply the token through Cargo's prompt, not in committed files or chat.
5. In `noms-core` Settings -> Trusted Publishing on crates.io, add:

   - Repository owner: `allypost-org`
   - Repository: `noms`
   - Workflow filename: `release.yml`
   - Environment: leave blank (the workflow does not use a GitHub environment)

6. Set GitHub repository Actions variable `PUBLISH_CRATE=true`. Revoke the initial
   API token once it is no longer needed. Future core release tags publish via OIDC.

Do not rerun the `0.1.0` tag's publishing job after enabling publication: that
version already exists. Routine releases always use a new version and tag.
No token secret is required in GitHub Actions.

## Routine releases

1. Update `[workspace.package].version` in root `Cargo.toml` and the workspace
   `noms-core` dependency requirement to the intended version. These are shared
   workspace versions; the unpublished applications advance with the core.
2. Update `crates/noms-core/CHANGELOG.md` and run
   `cargo update --workspace --offline` to refresh local package versions in
   `Cargo.lock`. Run the checks above, including the package consumer.
3. Commit and merge the release changes. On that clean commit, create an annotated
   tag matching the core manifest exactly:

   ```sh
   # Example after bumping the manifest to 0.1.1:
   scripts/check-core-release-tag noms-core-v0.1.1
   git tag -a noms-core-v0.1.1 -m 'noms-core 0.1.1'
   git push origin main noms-core-v0.1.1
   ```

4. `.github/workflows/release.yml` rejects mismatched tags, runs the complete CI
   workflow, publishes only `noms-core` with a short-lived crates.io token, then
   creates a GitHub release containing the verified `.crate`. It marks prerelease
   versions as GitHub prereleases. A failed publish prevents the GitHub release.

Restrict release-tag creation to maintainers. `PUBLISH_CRATE` controls registry
uploads; with it disabled, tags still run checks and create source-package GitHub
releases. Publishing or pushing tags is a release action, not part of local checks.
