# ADO

A CLI for opening browser tabs to different pages on the Azure Devops website relating to the repository at the current working directory.

## Common Commands

```bash
ado files
```

Opens the file explorer web page for the repository at the current working directory.

Also accepts a specific file path from the current directory: `ado files ./some/file.txt`

- Accepts the `--branch` option to override the source branch.
- Accepts the `--directory` option to override where the command is run from.

<hr />

```bash
ado pr
```

Opens the web page for the currently open pull request for the current branch in the repository at the current working directory.  
It will prompt if you'd like to open a pull request if one is not currently open.

- Accepts the `--branch` option to override the source branch.
- Accepts the `--directory` option to override where the command is run from.
- Accepts the `--create` flag to aways create a new pull request.

<hr />

```bash
ado item
```

Opens the PBI, Bug, Action etc. that is linked to the pull request currently open for the branch in the repository at the current working directory.

- Accepts the `--branch` option to override the source branch.
- Accepts the `--directory` option to override where the command is run from.
