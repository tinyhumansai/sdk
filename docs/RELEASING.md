# Releasing the SDKs

Run the **Release** workflow from GitHub Actions, select at least one SDK, and
choose a semantic-version bump. The workflow updates the selected package
versions on `main`, validates the selected packages, and creates a GitHub
Release for the resulting commit.

## Distribution targets

| SDK | GitHub distribution |
| --- | --- |
| TypeScript | Published as `@tinyhumansai/sdk` to GitHub Packages' npm registry |
| Python | Wheel and source distribution attached to the GitHub Release |
| Rust | Cargo `.crate` package attached to the GitHub Release |

GitHub Packages officially supports npm packages, but it does not provide
PyPI- or Cargo-compatible package registries. The Python and Rust outputs are
therefore downloadable GitHub Release artifacts, not PyPI or crates.io
publications.

The workflow uses its built-in `GITHUB_TOKEN`; repository secrets for npm,
PyPI, or crates.io are not required. Its TypeScript job has `packages: write`
permission, and the final release job has `contents: write` permission.

## Installing TypeScript from GitHub Packages

Authenticate npm to `npm.pkg.github.com` with a GitHub token that can read
packages, then configure the TinyHumans scope:

```ini
@tinyhumansai:registry=https://npm.pkg.github.com
//npm.pkg.github.com/:_authToken=${GITHUB_TOKEN}
```

Install the package normally after that configuration is in place:

```bash
npm install @tinyhumansai/sdk
```

Do not commit a token to `.npmrc`. The repository's checked-in
`sdk/typescript/.npmrc` contains only the non-secret scope-to-registry mapping.

## Using Python and Rust artifacts

Download the selected version's files from the repository's Releases page.
A Python wheel can be installed directly:

```bash
python -m pip install ./tinyhumans-<version>-py3-none-any.whl
```

The Rust `.crate` file is a packaged source artifact suitable for inspection
or use in a downstream private Cargo distribution. It is not installable from
GitHub through a crates.io-style registry URL.
