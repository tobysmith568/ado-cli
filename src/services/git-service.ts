import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { CliError } from "../cli/cli-error";

export class GitService {
  public findRepositoryRoot(startDirectory: string): string {
    const resolved = path.resolve(startDirectory);

    for (const current of this.ancestorDirectories(resolved)) {
      if (existsSync(path.join(current, ".git"))) {
        return current;
      }
    }

    throw new CliError("Not in a git repository");
  }

  public getRemoteUrl(repositoryRoot: string): string {
    const gitConfigPath = path.join(repositoryRoot, ".git", "config");

    let config: string;
    try {
      config = readFileSync(gitConfigPath, "utf8");
    } catch (error) {
      throw new CliError(
        `Cannot open or find the config file at ${gitConfigPath}: ${String(error)}`,
      );
    }

    const sections = config.split(/\r?\n\[/);

    for (const rawSection of sections) {
      const section = rawSection.startsWith("[")
        ? rawSection
        : `[${rawSection}`;

      if (!section.startsWith("[remote")) {
        continue;
      }

      const match = section.match(/\n\s*url\s*=\s*(.+)\s*/);
      if (match?.[1]) {
        return match[1].trim();
      }
    }

    throw new CliError("Cannot find remote url in the git config file");
  }

  public async getCurrentBranch(repositoryRoot: string): Promise<string> {
    const processResult = Bun.spawnSync(["git", "branch", "--show-current"], {
      cwd: repositoryRoot,
      stdout: "pipe",
      stderr: "pipe",
    });

    if (processResult.exitCode !== 0) {
      const stderr = new TextDecoder().decode(processResult.stderr).trim();
      throw new CliError(stderr || "Failed to get current git branch");
    }

    return new TextDecoder().decode(processResult.stdout).trim();
  }

  private *ancestorDirectories(start: string): Generator<string> {
    let current = start;

    while (true) {
      yield current;
      const parent = path.dirname(current);
      if (parent === current) {
        return;
      }
      current = parent;
    }
  }
}
