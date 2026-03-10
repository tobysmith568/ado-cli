import { mock as bunMock, describe, expect, it } from "bun:test";
import { createMock } from "mock-extended";
import type { AzureDevOpsClient } from "../services/azure-devops-client";
import type { BrowserService } from "../services/browser-service";
import type { RepositoryIdCache } from "../services/repository-id-cache";
import type { RepositoryService } from "../services/repository-service";
import type { InkPrompts } from "../ui/ink-prompts";
import { PrCommand } from "./pr-command";

describe("PrCommand", () => {
  const mock = createMock(() => bunMock());

  const repository = {
    organisationName: "org",
    projectName: "project",
    repositoryName: "repo",
    localPath: "/repo",
  };

  it("opens create URL when --create is set", async () => {
    const repositoryService = mock<RepositoryService>();
    const azureDevOpsClient = mock<AzureDevOpsClient>();
    const repositoryIdCache = mock<RepositoryIdCache>();
    const browserService = mock<BrowserService>();
    const prompts = mock<InkPrompts>();

    repositoryService.parseFromDirectory.mockReturnValue(repository);
    repositoryService.resolveBranchName.mockResolvedValue("main");
    repositoryService.getCreatePrUrlForBranch.mockReturnValue(
      "https://example.dev/create",
    );

    const command = new PrCommand(
      repositoryService as unknown as RepositoryService,
      azureDevOpsClient as unknown as AzureDevOpsClient,
      repositoryIdCache as unknown as RepositoryIdCache,
      browserService as unknown as BrowserService,
      prompts as unknown as InkPrompts,
    );

    await command.execute({ create: true, directory: "/repo" });

    expect(browserService.open).toHaveBeenCalledWith(
      "https://example.dev/create",
    );
  });

  it("opens existing pull request URL when PR exists", async () => {
    const repositoryService = mock<RepositoryService>();
    const azureDevOpsClient = mock<AzureDevOpsClient>();
    const repositoryIdCache = mock<RepositoryIdCache>();
    const browserService = mock<BrowserService>();
    const prompts = mock<InkPrompts>();

    repositoryService.parseFromDirectory.mockReturnValue(repository);
    repositoryService.resolveBranchName.mockResolvedValue(
      "feature/existing-pr",
    );
    repositoryService.getPullRequestUrl.mockReturnValue(
      "https://example.dev/pr/42",
    );
    repositoryIdCache.resolve.mockImplementation(async (_root, fetcher) =>
      fetcher(),
    );
    azureDevOpsClient.getRepositoryId.mockResolvedValue("repo-id");
    azureDevOpsClient.getOpenPrIdForBranch.mockResolvedValue(42);

    const command = new PrCommand(
      repositoryService as unknown as RepositoryService,
      azureDevOpsClient as unknown as AzureDevOpsClient,
      repositoryIdCache as unknown as RepositoryIdCache,
      browserService as unknown as BrowserService,
      prompts as unknown as InkPrompts,
    );

    await command.execute({ directory: "/repo" });

    expect(repositoryService.getPullRequestUrl).toHaveBeenCalledWith(
      repository,
      42,
    );
    expect(browserService.open).toHaveBeenCalledWith(
      "https://example.dev/pr/42",
    );
    expect(prompts.confirm).not.toHaveBeenCalled();
  });

  it("opens create PR URL when no PR exists and user confirms", async () => {
    const repositoryService = mock<RepositoryService>();
    const azureDevOpsClient = mock<AzureDevOpsClient>();
    const repositoryIdCache = mock<RepositoryIdCache>();
    const browserService = mock<BrowserService>();
    const prompts = mock<InkPrompts>();

    repositoryService.parseFromDirectory.mockReturnValue(repository);
    repositoryService.resolveBranchName.mockResolvedValue("feature/new-pr");
    repositoryService.getCreatePrUrlForBranch.mockReturnValue(
      "https://example.dev/create",
    );
    repositoryIdCache.resolve.mockImplementation(async (_root, fetcher) =>
      fetcher(),
    );
    azureDevOpsClient.getRepositoryId.mockResolvedValue("repo-id");
    azureDevOpsClient.getOpenPrIdForBranch.mockResolvedValue(undefined);
    prompts.confirm.mockResolvedValue(true);

    const command = new PrCommand(
      repositoryService as unknown as RepositoryService,
      azureDevOpsClient as unknown as AzureDevOpsClient,
      repositoryIdCache as unknown as RepositoryIdCache,
      browserService as unknown as BrowserService,
      prompts as unknown as InkPrompts,
    );

    await command.execute({ directory: "/repo" });

    expect(repositoryService.getCreatePrUrlForBranch).toHaveBeenCalledWith(
      repository,
      "feature/new-pr",
    );
    expect(browserService.open).toHaveBeenCalledWith(
      "https://example.dev/create",
    );
  });

  it("does not open browser when no PR exists and user declines prompt", async () => {
    const repositoryService = mock<RepositoryService>();
    const azureDevOpsClient = mock<AzureDevOpsClient>();
    const repositoryIdCache = mock<RepositoryIdCache>();
    const browserService = mock<BrowserService>();
    const prompts = mock<InkPrompts>();

    repositoryService.parseFromDirectory.mockReturnValue(repository);
    repositoryService.resolveBranchName.mockResolvedValue("feature/no-create");
    repositoryIdCache.resolve.mockImplementation(async (_root, fetcher) =>
      fetcher(),
    );
    azureDevOpsClient.getRepositoryId.mockResolvedValue("repo-id");
    azureDevOpsClient.getOpenPrIdForBranch.mockResolvedValue(undefined);
    prompts.confirm.mockResolvedValue(false);

    const command = new PrCommand(
      repositoryService as unknown as RepositoryService,
      azureDevOpsClient as unknown as AzureDevOpsClient,
      repositoryIdCache as unknown as RepositoryIdCache,
      browserService as unknown as BrowserService,
      prompts as unknown as InkPrompts,
    );

    await command.execute({ directory: "/repo" });

    expect(browserService.open).not.toHaveBeenCalled();
  });
});
