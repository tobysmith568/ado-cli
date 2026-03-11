import { existsSync, readFileSync, writeFileSync } from "node:fs";
import path from "node:path";

type RepositoryIdResolver = () => Promise<string>;

export class RepositoryIdCache {
  public async resolve(
    repositoryRoot: string,
    fetcher: RepositoryIdResolver,
  ): Promise<string> {
    const cachePath = path.join(repositoryRoot, ".git", "ado_repo_id");

    if (existsSync(cachePath)) {
      const id = readFileSync(cachePath, "utf8").trim();
      if (id) {
        return id;
      }
    }

    const repositoryId = await fetcher();
    writeFileSync(cachePath, repositoryId);
    return repositoryId;
  }
}
