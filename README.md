# ADO CLI (V2)

A CLI for opening browser tabs to Azure DevOps pages related to the repository in your current working directory.

## Install / Build

Requirements:
- Bun 1.3+
- Git

Local dev:

```bash
bun install
bun run lint
bun run typecheck
bun test
```

Build executable:

```bash
bun run build
```

This outputs a compiled binary at `dist/ado` (or `dist/ado.exe` on Windows).

## Commands

### `ado files [file_path]`
Open the Azure DevOps file explorer for the current branch.

Examples:

```bash
ado files
ado files ./src/services/repository-service.ts
```

Options:
- `-b, --branch <branch>`: override branch
- `-d, --directory <directory>`: run command as if from another directory

Alias:
- `ado file`

### `ado pr`
Open the currently active pull request for the branch. If no active PR exists, prompts to open the create-PR page.

Options:
- `-b, --branch <branch>`: override branch
- `-d, --directory <directory>`: run command as if from another directory
- `-c, --create`: always open the create-PR page

### `ado item`
Open the first work item linked to the active PR for the branch.

Options:
- `-b, --branch <branch>`: override branch
- `-d, --directory <directory>`: run command as if from another directory
- `--id <id>`: open work item directly (skip PR lookup)

Aliases:
- `ado pbi`
- `ado bug`

## Auth Configuration

Credentials are stored in `~/.ado_cli` (parity with V1).

Supported modes:
- store PAT directly in config
- store an environment variable name in config (reads PAT value from env var)

On first authenticated command, the CLI prompts you to select a mode.

## CI / Release

Workflows:
- `.github/workflows/integration.yml`: lint, typecheck, test, and multi-OS build artifacts
- `.github/workflows/deployment.yml`: manual versioned release + optional Chocolatey publish
- `.github/workflows/codeql.yml`: static analysis scanning
