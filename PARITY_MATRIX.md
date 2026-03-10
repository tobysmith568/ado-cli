# Parity Matrix (V1 -> V2)

This matrix tracks user-visible behavior from Rust V1 against TypeScript/Bun V2 implementation and tests.

## Command Surface

| V1 Behavior | V2 Status | Implementation | Test Coverage |
|---|---|---|---|
| `ado files [file_path]` command exists | Complete | `src/commands/files-command.ts` | `src/commands/files-command.test.ts` |
| `ado file` alias for `files` | Complete | `src/cli/main.ts` | Covered via command registration in runtime |
| `ado pr` command exists | Complete | `src/commands/pr-command.ts` | `src/commands/pr-command.test.ts` |
| `ado item` command exists | Complete | `src/commands/item-command.ts` | `src/commands/item-command.test.ts` |
| `ado pbi` and `ado bug` aliases for `item` | Complete | `src/cli/main.ts` | Covered via command registration in runtime |

## Files Command Behavior

| V1 Behavior | V2 Status | Implementation | Test Coverage |
|---|---|---|---|
| Uses current branch if `--branch` not set | Complete | `src/services/repository-service.ts` | `src/commands/files-command.test.ts` |
| Supports `--directory` working directory override | Complete | `src/commands/files-command.ts` | `src/commands/files-command.test.ts` |
| Opens branch explorer URL when no path provided | Complete | `src/services/repository-service.ts` | `src/commands/files-command.test.ts` |
| Opens file explorer URL for specific path | Complete | `src/services/repository-service.ts` | `src/commands/files-command.test.ts` |
| Rejects path outside repository root | Complete | `src/services/repository-service.ts` | `src/services/repository-service.test.ts` |

## PR Command Behavior

| V1 Behavior | V2 Status | Implementation | Test Coverage |
|---|---|---|---|
| `--create` opens create PR page directly | Complete | `src/commands/pr-command.ts` | `src/commands/pr-command.test.ts` |
| Opens existing active PR when present | Complete | `src/commands/pr-command.ts` | `src/commands/pr-command.test.ts` |
| Prompts to create PR when none exists | Complete | `src/commands/pr-command.ts` + `src/ui/ink-prompts.tsx` | `src/commands/pr-command.test.ts` |
| No-op when user declines create prompt | Complete | `src/commands/pr-command.ts` | `src/commands/pr-command.test.ts` |
| Branch lookup uses normalized full refs | Complete (improved) | `src/services/azure-devops-client.ts` | `src/services/azure-devops-client.test.ts` |

## Item Command Behavior

| V1 Behavior | V2 Status | Implementation | Test Coverage |
|---|---|---|---|
| `--id` opens work item directly | Complete | `src/commands/item-command.ts` | `src/commands/item-command.test.ts` |
| Errors if no PR exists for branch | Complete | `src/commands/item-command.ts` | `src/commands/item-command.test.ts` |
| Opens first linked work item when present | Complete | `src/commands/item-command.ts` | `src/commands/item-command.test.ts` |
| Silent success when PR has no linked work items | Complete | `src/commands/item-command.ts` | `src/commands/item-command.test.ts` |

## Auth and Config

| V1 Behavior | V2 Status | Implementation | Test Coverage |
|---|---|---|---|
| PAT retrieval from config (`~/.ado_cli`) | Complete | `src/services/api-key-manager.ts` + `src/services/config-store.ts` | `src/services/api-key-manager.test.ts` |
| Env-var reference mode (`credential.env_var`) | Complete | `src/services/api-key-manager.ts` | `src/services/api-key-manager.test.ts` |
| First-run prompt for storage mode | Complete | `src/services/api-key-manager.ts` + `src/ui/ink-prompts.tsx` | `src/services/api-key-manager.test.ts` |
| Non-TTY confirm safety defaults to No | Complete (improved) | `src/ui/ink-prompts.tsx` | `src/ui/ink-prompts.test.ts` |

## URL and Repository Resolution

| V1 Behavior | V2 Status | Implementation | Test Coverage |
|---|---|---|---|
| Parse HTTPS ADO remotes | Complete | `src/services/remote-url-parser.ts` | `src/services/remote-url-parser.test.ts` |
| Parse SSH ADO remotes | Complete | `src/services/remote-url-parser.ts` | `src/services/remote-url-parser.test.ts` |
| URL templates for files/PR/work item pages | Complete | `src/services/repository-service.ts` | `src/services/repository-service.test.ts` |
| Cache repository ID at `.git/ado_repo_id` | Complete | `src/services/repository-id-cache.ts` | `src/services/repository-id-cache.test.ts` |

## CI/CD and Security

| V1 Contract | V2 Status | Implementation |
|---|---|---|
| CI lint/test/build on PR and main | Complete | `.github/workflows/integration.yml` |
| Manual versioned deployment workflow | Complete | `.github/workflows/deployment.yml` |
| Security scanning workflow | Complete | `.github/workflows/codeql.yml` |
| Chocolatey packaging path retained | Complete | `choco/ado/ado.nuspec` + `.github/workflows/deployment.yml` |

## Remaining Operational Step

- Dry-run execution of the deployment workflow in GitHub Actions for a real version tag and artifact promotion validation.
  - This requires repository Actions execution context and cannot be fully completed from local CLI-only execution.
