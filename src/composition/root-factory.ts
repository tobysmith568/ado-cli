import { FilesCommand } from "../commands/files-command";
import { ItemCommand } from "../commands/item-command";
import { PrCommand } from "../commands/pr-command";
import { ApiKeyManager } from "../services/api-key-manager";
import { AzureDevOpsClient } from "../services/azure-devops-client";
import { BrowserService } from "../services/browser-service";
import { ConfigStore } from "../services/config-store";
import { GitService } from "../services/git-service";
import { RemoteUrlParser } from "../services/remote-url-parser";
import { RepositoryIdCache } from "../services/repository-id-cache";
import { RepositoryService } from "../services/repository-service";
import { InkPrompts } from "../ui/ink-prompts";

export type RootContainer = {
  filesCommand: FilesCommand;
  prCommand: PrCommand;
  itemCommand: ItemCommand;
};

export async function buildRootContainer(): Promise<RootContainer> {
  const configStore = new ConfigStore();
  const prompts = new InkPrompts();
  const apiKeyManager = new ApiKeyManager(configStore, prompts);
  const apiKey = await apiKeyManager.getApiKey();

  const browserService = new BrowserService();
  const gitService = new GitService();
  const remoteUrlParser = new RemoteUrlParser();
  const repositoryService = new RepositoryService(gitService, remoteUrlParser);

  const azureDevOpsClient = new AzureDevOpsClient(apiKey);
  const repositoryIdCache = new RepositoryIdCache();

  return {
    filesCommand: new FilesCommand(repositoryService, browserService),
    prCommand: new PrCommand(
      repositoryService,
      azureDevOpsClient,
      repositoryIdCache,
      browserService,
      prompts,
    ),
    itemCommand: new ItemCommand(
      repositoryService,
      azureDevOpsClient,
      repositoryIdCache,
      browserService,
    ),
  };
}
