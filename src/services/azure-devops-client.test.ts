import { mock as bunMock, describe, expect, it } from "bun:test";
import { createMock } from "mock-extended";
import { CliError } from "../cli/cli-error";
import { AzureDevOpsClient } from "./azure-devops-client";

type GitApiForPrLookup = {
  getPullRequests: (
    repositoryId: string,
    criteria: { sourceRefName?: string },
    projectName: string,
  ) => Promise<Array<{ pullRequestId?: number }>>;
};

type GitApiForRepository = {
  getRepository: () => Promise<{ id?: string }>;
};

type GitApiForWorkItems = {
  getPullRequestWorkItemRefs: () => Promise<Array<{ id?: string }>>;
};

describe("AzureDevOpsClient", () => {
  const mock = createMock(() => bunMock());

  it("normalizes plain branch names to full refs for PR lookup", async () => {
    const client = new AzureDevOpsClient("pat") as unknown as {
      getOpenPrIdForBranch: AzureDevOpsClient["getOpenPrIdForBranch"];
      getGitApi: () => Promise<{
        getPullRequests: (
          repositoryId: string,
          criteria: { sourceRefName?: string },
          projectName: string,
        ) => Promise<Array<{ pullRequestId?: number }>>;
      }>;
    };

    let capturedSourceRef = "";

    const gitApi = mock<GitApiForPrLookup>();
    gitApi.getPullRequests.mockImplementation(
      async (_repositoryId, criteria) => {
        capturedSourceRef = criteria.sourceRefName ?? "";
        return [{ pullRequestId: 7 }];
      },
    );

    client.getGitApi = async () => gitApi;

    const prId = await client.getOpenPrIdForBranch(
      "org",
      "project",
      "repo-id",
      "feature/demo",
    );

    expect(prId).toBe(7);
    expect(capturedSourceRef).toBe("refs/heads/feature/demo");
  });

  it("maps auth failures to a clear CliError message", async () => {
    const client = new AzureDevOpsClient("bad-pat") as unknown as {
      getRepositoryId: AzureDevOpsClient["getRepositoryId"];
      getGitApi: () => Promise<{
        getRepository: () => Promise<{ id?: string }>;
      }>;
    };

    const gitApi = mock<GitApiForRepository>();
    gitApi.getRepository.mockRejectedValue(new Error("401 Unauthorized"));

    client.getGitApi = async () => gitApi;

    await expect(
      client.getRepositoryId("org", "project", "repo"),
    ).rejects.toBeInstanceOf(CliError);
    await expect(
      client.getRepositoryId("org", "project", "repo"),
    ).rejects.toThrow("authentication failed");
  });

  it("maps not-found failures to a clear CliError message", async () => {
    const client = new AzureDevOpsClient("pat") as unknown as {
      getPullRequestWorkItemIds: AzureDevOpsClient["getPullRequestWorkItemIds"];
      getGitApi: () => Promise<{
        getPullRequestWorkItemRefs: () => Promise<Array<{ id?: string }>>;
      }>;
    };

    const gitApi = mock<GitApiForWorkItems>();
    gitApi.getPullRequestWorkItemRefs.mockRejectedValue(
      new Error("404 Not Found"),
    );

    client.getGitApi = async () => gitApi;

    await expect(
      client.getPullRequestWorkItemIds("org", "project", "repo", 1),
    ).rejects.toBeInstanceOf(CliError);
    await expect(
      client.getPullRequestWorkItemIds("org", "project", "repo", 1),
    ).rejects.toThrow("resource not found");
  });
});
