# Release Dry-Run Checklist

Use this checklist before the first production V2 release.

## Preconditions

- `main` branch is green.
- `bun run lint && bun run typecheck && bun test` passes locally.
- `integration.yml` has succeeded on latest commit.
- `choco_api_key` secret exists in repository settings if Chocolatey publish is desired.

## Manual Dispatch

1. Open Actions -> `Deployment` workflow.
2. Click `Run workflow` on `main`.
3. Enter `version` as SemVer without leading `v` (example `1.0.0`).
4. Start workflow.

## Validate Integration Reuse

- Confirm `integration` job ran via reusable workflow call.
- Confirm artifacts exist:
  - `ado-ubuntu-latest`
  - `ado-macos-latest`
  - `ado-windows-latest`

## Validate GitHub Release

- Confirm `create-release` job succeeds.
- Confirm tag `v<version>` created.
- Confirm release assets uploaded:
  - `Ubuntu-Executable.zip`
  - `MacOS-Executable.zip`
  - `Windows-Executable.zip`
- Open each zip and ensure expected binary exists (`ado` or `ado.exe`).

## Validate Chocolatey (optional)

- If `choco_api_key` secret is set, confirm `choco` job ran.
- Confirm nuspec placeholders were replaced in workflow execution.
- Confirm `.nupkg` built successfully.
- Confirm package push succeeded on `https://push.chocolatey.org/`.

## Post-Run Verification

- Install from GitHub release binary and run:
  - `ado files`
  - `ado pr`
  - `ado item --id 1`
- If Chocolatey publish ran, validate install command:
  - `choco install ado --version=<version>`

## Failure Recovery

- If release creation fails: fix workflow/artifact naming and re-run with same version after deleting broken tag/release.
- If Chocolatey publish fails after release succeeds: re-run only Chocolatey portion manually once issue is fixed.
- If artifact mismatch occurs: verify `integration.yml` upload names match `deployment.yml` download names.
