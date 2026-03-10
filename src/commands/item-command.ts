import { CliError } from "../cli/cli-error";
import type { AzureDevOpsClient } from "../services/azure-devops-client";
import type { BrowserService } from "../services/browser-service";
import type { RepositoryIdCache } from "../services/repository-id-cache";
import type { RepositoryService } from "../services/repository-service";

export type ItemCommandArgs = {
  directory?: string;
  branch?: string;
  id?: string;
};

export class ItemCommand {
  public constructor(
    private readonly repositoryService: RepositoryService,
    private readonly azureDevOpsClient: AzureDevOpsClient,
    private readonly repositoryIdCache: RepositoryIdCache,
    private readonly browserService: BrowserService,
  ) {}

  public async execute(args: ItemCommandArgs): Promise<void> {
    const workingDirectory = args.directory ?? process.cwd();
    const repository =
      this.repositoryService.parseFromDirectory(workingDirectory);

    if (args.id) {
      const workItemUrl = this.repositoryService.getWorkItemUrl(
        repository,
        args.id,
      );
      await this.browserService.open(workItemUrl);
      return;
    }

    const branchName = await this.repositoryService.resolveBranchName(
      repository.localPath,
      args.branch,
    );

    const repositoryId = await this.repositoryIdCache.resolve(
      repository.localPath,
      async () =>
        this.azureDevOpsClient.getRepositoryId(
          repository.organisationName,
          repository.projectName,
          repository.repositoryName,
        ),
    );

    const prId = await this.azureDevOpsClient.getOpenPrIdForBranch(
      repository.organisationName,
      repository.projectName,
      repositoryId,
      branchName,
    );

    if (!prId) {
      throw new CliError(
        `There is no open PR for the branch ${branchName}; so a linked PBI/Bug/etc. could not be discovered.`,
      );
    }

    const workItemIds = await this.azureDevOpsClient.getPullRequestWorkItemIds(
      repository.organisationName,
      repository.projectName,
      repository.repositoryName,
      prId,
    );

    const firstWorkItemId = workItemIds[0];

    if (!firstWorkItemId) {
      return;
    }

    const workItemUrl = this.repositoryService.getWorkItemUrl(
      repository,
      firstWorkItemId,
    );
    await this.browserService.open(workItemUrl);
  }
}
