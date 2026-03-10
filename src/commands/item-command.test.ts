import { mock as bunMock, describe, expect, it } from "bun:test";
import { createMock } from "mock-extended";
import { CliError } from "../cli/cli-error";
import type { AzureDevOpsClient } from "../services/azure-devops-client";
import type { BrowserService } from "../services/browser-service";
import type { RepositoryIdCache } from "../services/repository-id-cache";
import type { RepositoryService } from "../services/repository-service";
import { ItemCommand } from "./item-command";

describe("ItemCommand", () => {
  const mock = createMock(() => bunMock());

  const repository = {
    organisationName: "org",
    projectName: "project",
    repositoryName: "repo",
    localPath: "/repo",
  };

  it("opens work item directly when --id is provided", async () => {
    const repositoryService = mock<RepositoryService>();
    const azureDevOpsClient = mock<AzureDevOpsClient>();
    const repositoryIdCache = mock<RepositoryIdCache>();
    const browserService = mock<BrowserService>();

    repositoryService.parseFromDirectory.mockReturnValue(repository);
    repositoryService.getWorkItemUrl.mockReturnValue(
      "https://example.dev/item/100",
    );

    const command = new ItemCommand(
      repositoryService as unknown as RepositoryService,
      azureDevOpsClient as unknown as AzureDevOpsClient,
      repositoryIdCache as unknown as RepositoryIdCache,
      browserService as unknown as BrowserService,
    );

    await command.execute({ directory: "/repo", id: "100" });

    expect(browserService.open).toHaveBeenCalledWith(
      "https://example.dev/item/100",
    );
    expect(azureDevOpsClient.getOpenPrIdForBranch).not.toHaveBeenCalled();
  });

  it("throws when no PR exists for branch", async () => {
    const repositoryService = mock<RepositoryService>();
    const azureDevOpsClient = mock<AzureDevOpsClient>();
    const repositoryIdCache = mock<RepositoryIdCache>();
    const browserService = mock<BrowserService>();

    repositoryService.parseFromDirectory.mockReturnValue(repository);
    repositoryService.resolveBranchName.mockResolvedValue("feature/no-pr");
    repositoryIdCache.resolve.mockImplementation(async (_root, fetcher) =>
      fetcher(),
    );
    azureDevOpsClient.getRepositoryId.mockResolvedValue("repo-id");
    azureDevOpsClient.getOpenPrIdForBranch.mockResolvedValue(undefined);

    const command = new ItemCommand(
      repositoryService as unknown as RepositoryService,
      azureDevOpsClient as unknown as AzureDevOpsClient,
      repositoryIdCache as unknown as RepositoryIdCache,
      browserService as unknown as BrowserService,
    );

    await expect(
      command.execute({ directory: "/repo" }),
    ).rejects.toBeInstanceOf(CliError);
    expect(browserService.open).not.toHaveBeenCalled();
  });

  it("opens first linked work item when PR has linked items", async () => {
    const repositoryService = mock<RepositoryService>();
    const azureDevOpsClient = mock<AzureDevOpsClient>();
    const repositoryIdCache = mock<RepositoryIdCache>();
    const browserService = mock<BrowserService>();

    repositoryService.parseFromDirectory.mockReturnValue(repository);
    repositoryService.resolveBranchName.mockResolvedValue("feature/with-item");
    repositoryService.getWorkItemUrl.mockReturnValue(
      "https://example.dev/item/55",
    );
    repositoryIdCache.resolve.mockImplementation(async (_root, fetcher) =>
      fetcher(),
    );
    azureDevOpsClient.getRepositoryId.mockResolvedValue("repo-id");
    azureDevOpsClient.getOpenPrIdForBranch.mockResolvedValue(42);
    azureDevOpsClient.getPullRequestWorkItemIds.mockResolvedValue(["55", "56"]);

    const command = new ItemCommand(
      repositoryService as unknown as RepositoryService,
      azureDevOpsClient as unknown as AzureDevOpsClient,
      repositoryIdCache as unknown as RepositoryIdCache,
      browserService as unknown as BrowserService,
    );

    await command.execute({ directory: "/repo" });

    expect(repositoryService.getWorkItemUrl).toHaveBeenCalledWith(
      repository,
      "55",
    );
    expect(browserService.open).toHaveBeenCalledWith(
      "https://example.dev/item/55",
    );
  });

  it("silently exits when PR has no linked work items", async () => {
    const repositoryService = mock<RepositoryService>();
    const azureDevOpsClient = mock<AzureDevOpsClient>();
    const repositoryIdCache = mock<RepositoryIdCache>();
    const browserService = mock<BrowserService>();

    repositoryService.parseFromDirectory.mockReturnValue(repository);
    repositoryService.resolveBranchName.mockResolvedValue("feature/no-items");
    repositoryIdCache.resolve.mockImplementation(async (_root, fetcher) =>
      fetcher(),
    );
    azureDevOpsClient.getRepositoryId.mockResolvedValue("repo-id");
    azureDevOpsClient.getOpenPrIdForBranch.mockResolvedValue(101);
    azureDevOpsClient.getPullRequestWorkItemIds.mockResolvedValue([]);

    const command = new ItemCommand(
      repositoryService as unknown as RepositoryService,
      azureDevOpsClient as unknown as AzureDevOpsClient,
      repositoryIdCache as unknown as RepositoryIdCache,
      browserService as unknown as BrowserService,
    );

    await command.execute({ directory: "/repo" });

    expect(browserService.open).not.toHaveBeenCalled();
  });
});
