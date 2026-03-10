import { describe, expect, it } from "bun:test";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import os from "node:os";
import path from "node:path";
import { RepositoryIdCache } from "./repository-id-cache";

describe("RepositoryIdCache", () => {
  it("returns cached repository id when cache file exists", async () => {
    const root = mkdtempSync(path.join(os.tmpdir(), "ado-cache-hit-"));
    const gitDir = path.join(root, ".git");
    mkdirSync(gitDir, { recursive: true });
    writeFileSync(path.join(gitDir, "ado_repo_id"), "cached-id\n");

    const cache = new RepositoryIdCache();
    let fetcherCalled = false;

    const value = await cache.resolve(root, async () => {
      fetcherCalled = true;
      return "fetched-id";
    });

    expect(value).toBe("cached-id");
    expect(fetcherCalled).toBe(false);
  });

  it("uses fetcher and writes cache when cache file is missing", async () => {
    const root = mkdtempSync(path.join(os.tmpdir(), "ado-cache-miss-"));
    const gitDir = path.join(root, ".git");
    mkdirSync(gitDir, { recursive: true });

    const cache = new RepositoryIdCache();

    const value = await cache.resolve(root, async () => "fetched-id");

    expect(value).toBe("fetched-id");

    const cacheFile = path.join(gitDir, "ado_repo_id");
    const fileContents = await Bun.file(cacheFile).text();
    expect(fileContents).toBe("fetched-id");
  });
});
