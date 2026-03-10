import { describe, expect, it } from "bun:test";
import { RemoteUrlParser } from "./remote-url-parser";

describe("RemoteUrlParser", () => {
  const parser = new RemoteUrlParser();

  it("parses http remotes with username", () => {
    const parsed = parser.parse(
      "https://user@dev.azure.com/org/project/_git/repo",
    );

    expect(parsed.organisationName).toBe("org");
    expect(parsed.projectName).toBe("project");
    expect(parsed.repositoryName).toBe("repo");
  });

  it("parses ssh remotes", () => {
    const parsed = parser.parse("git@ssh.dev.azure.com:v3/org/project/repo");

    expect(parsed.organisationName).toBe("org");
    expect(parsed.projectName).toBe("project");
    expect(parsed.repositoryName).toBe("repo");
  });
});
