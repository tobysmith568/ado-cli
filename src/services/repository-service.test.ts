import { mock as bunMock, describe, expect, it } from "bun:test";
import { createMock } from "mock-extended";
import { CliError } from "../cli/cli-error";
import type { GitService } from "./git-service";
import type { RemoteUrlParser } from "./remote-url-parser";
import { RepositoryService } from "./repository-service";

describe("RepositoryService", () => {
  const mock = createMock(() => bunMock());
  const gitService = mock<GitService>();
  const remoteUrlParser = mock<RemoteUrlParser>();
  const service = new RepositoryService(
    gitService as unknown as GitService,
    remoteUrlParser as unknown as RemoteUrlParser,
  );
  const context = {
    organisationName: "org",
    projectName: "project",
    repositoryName: "repo",
    localPath: "/repo",
  };

  it("parses repository context from directory using injected dependencies", () => {
    gitService.findRepositoryRoot.mockReturnValue("/repo");
    gitService.getRemoteUrl.mockReturnValue(
      "https://user@dev.azure.com/org/project/_git/repo",
    );
    remoteUrlParser.parse.mockReturnValue({
      organisationName: "org",
      projectName: "project",
      repositoryName: "repo",
    });

    const parsed = service.parseFromDirectory("/repo/src");

    expect(gitService.findRepositoryRoot).toHaveBeenCalledWith("/repo/src");
    expect(gitService.getRemoteUrl).toHaveBeenCalledWith("/repo");
    expect(remoteUrlParser.parse).toHaveBeenCalledWith(
      "https://user@dev.azure.com/org/project/_git/repo",
    );
    expect(parsed).toEqual(context);
  });

  it("builds file URL with encoded branch and path", () => {
    const url = service.getFilesUrlForFileOnBranch(
      context,
      "feature/my branch",
      "src/my file.ts",
      "/repo",
    );

    expect(url).toBe(
      "https://dev.azure.com/org/project/_git/repo?version=GBfeature%2Fmy%20branch&path=%2Fsrc%2Fmy%20file.ts",
    );
  });

  it("throws when file path resolves outside repository root", () => {
    expect(() =>
      service.getFilesUrlForFileOnBranch(
        context,
        "main",
        "../outside.ts",
        "/repo",
      ),
    ).toThrow(CliError);
  });

  it("matches files URL template contract for branch view", () => {
    const url = service.getFilesUrlForBranch(
      context,
      "feature/contract branch",
    );

    expect(url).toBe(
      "https://dev.azure.com/org/project/_git/repo?version=GBfeature%2Fcontract%20branch",
    );
  });

  it("matches create PR URL template contract", () => {
    const url = service.getCreatePrUrlForBranch(
      context,
      "feature/contract branch",
    );

    expect(url).toBe(
      "https://dev.azure.com/org/project/_git/repo/pullrequestcreate?sourceRef=feature%2Fcontract%20branch",
    );
  });

  it("matches pull request URL template contract", () => {
    const url = service.getPullRequestUrl(context, 123);

    expect(url).toBe(
      "https://dev.azure.com/org/project/_git/repo/pullrequest/123",
    );
  });

  it("matches work item URL template contract", () => {
    const url = service.getWorkItemUrl(context, "456");

    expect(url).toBe("https://dev.azure.com/org/project/_workitems/edit/456");
  });

  it("resolves file path relative to provided working directory", () => {
    const url = service.getFilesUrlForFileOnBranch(
      context,
      "main",
      "child/file.ts",
      "/repo/src",
    );

    expect(url).toBe(
      "https://dev.azure.com/org/project/_git/repo?version=GBmain&path=%2Fsrc%2Fchild%2Ffile.ts",
    );
  });
});
