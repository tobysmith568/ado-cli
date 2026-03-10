# Migration Notes (Rust V1 -> TypeScript/Bun V2)

## Preserved Behavior

- Command surface and aliases:
  - `files` (`file`)
  - `pr`
  - `item` (`pbi`, `bug`)
- `--branch` and `--directory` behavior
- Work item lookup via branch PR
- Credential storage model in `~/.ado_cli`
- Repository ID cache at `.git/ado_repo_id`

## Intentional Fixes / Improvements

- PR branch matching uses normalized full refs (`refs/heads/...`) to avoid suffix false positives.
- Non-interactive prompt safety:
  - `confirm()` defaults to `false` in non-TTY mode.
- More explicit Azure DevOps API error mapping for auth/not-found cases.
- Test layout moved to co-located `*.test.ts` files beside implementation files.

## New Tooling

- Runtime/build/test: Bun
- Language: TypeScript (strict)
- CLI parsing: `commander` + `@commander-js/extra-typings`
- Interactive UI: Ink
- Lint/format: Biome
- API client: `azure-devops-node-api`
- Test mocks: `mock-extended`
