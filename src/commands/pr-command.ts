import type { AzureDevOpsClient } from "../services/azure-devops-client";
import type { BrowserService } from "../services/browser-service";
import type { RepositoryIdCache } from "../services/repository-id-cache";
import type { RepositoryService } from "../services/repository-service";
import type { InkPrompts } from "../ui/ink-prompts";

export type PrCommandArgs = {
  directory?: string;
  branch?: string;
  create?: boolean;
};

export class PrCommand {
  public constructor(
    private readonly repositoryService: RepositoryService,
    private readonly azureDevOpsClient: AzureDevOpsClient,
    private readonly repositoryIdCache: RepositoryIdCache,
    private readonly browserService: BrowserService,
    private readonly prompts: InkPrompts,
  ) {}

  public async execute(args: PrCommandArgs): Promise<void> {
    const workingDirectory = args.directory ?? process.cwd();
    const repository =
      this.repositoryService.parseFromDirectory(workingDirectory);
    const branchName = await this.repositoryService.resolveBranchName(
      repository.localPath,
      args.branch,
    );

    if (args.create) {
      const createUrl = this.repositoryService.getCreatePrUrlForBranch(
        repository,
        branchName,
      );
      await this.browserService.open(createUrl);
      return;
    }

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
      const shouldCreate = await this.prompts.confirm(
        `There is no open PR for branch ${branchName}. Would you like to open one?`,
      );

      if (shouldCreate) {
        const createUrl = this.repositoryService.getCreatePrUrlForBranch(
          repository,
          branchName,
        );
        await this.browserService.open(createUrl);
      }

      return;
    }

    const prUrl = this.repositoryService.getPullRequestUrl(repository, prId);
    await this.browserService.open(prUrl);
  }
}
