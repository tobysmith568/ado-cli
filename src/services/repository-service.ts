import path from 'node:path';
import { CliError } from '../cli/cli-error';
import type { RepositoryContext } from '../domain/repository-context';
import type { GitService } from './git-service';
import type { RemoteUrlParser } from './remote-url-parser';

function urlEncode(value: string): string {
  return encodeURIComponent(value);
}

export class RepositoryService {
  public constructor(
    private readonly gitService: GitService,
    private readonly remoteUrlParser: RemoteUrlParser,
  ) {}

  public parseFromDirectory(directory: string): RepositoryContext {
    const repositoryRoot = this.gitService.findRepositoryRoot(directory);
    const remoteUrl = this.gitService.getRemoteUrl(repositoryRoot);
    const parsed = this.remoteUrlParser.parse(remoteUrl);

    return {
      ...parsed,
      localPath: repositoryRoot,
    };
  }

  public async resolveBranchName(repositoryRoot: string, branch?: string): Promise<string> {
    if (branch) {
      return branch;
    }

    return this.gitService.getCurrentBranch(repositoryRoot);
  }

  public getFilesUrlForBranch(context: RepositoryContext, branchName: string): string {
    return `https://dev.azure.com/${context.organisationName}/${context.projectName}/_git/${context.repositoryName}?version=GB${urlEncode(branchName)}`;
  }

  public getFilesUrlForFileOnBranch(
    context: RepositoryContext,
    branchName: string,
    fileOrDirectory: string,
    workingDirectory: string,
  ): string {
    const filePath = this.formatAdoFilePath(context.localPath, fileOrDirectory, workingDirectory);

    return `https://dev.azure.com/${context.organisationName}/${context.projectName}/_git/${context.repositoryName}?version=GB${urlEncode(branchName)}&path=${urlEncode(filePath)}`;
  }

  public getCreatePrUrlForBranch(context: RepositoryContext, branchName: string): string {
    return `https://dev.azure.com/${context.organisationName}/${context.projectName}/_git/${context.repositoryName}/pullrequestcreate?sourceRef=${urlEncode(branchName)}`;
  }

  public getPullRequestUrl(context: RepositoryContext, prId: number): string {
    return `https://dev.azure.com/${context.organisationName}/${context.projectName}/_git/${context.repositoryName}/pullrequest/${prId}`;
  }

  public getWorkItemUrl(context: RepositoryContext, workItemId: string): string {
    return `https://dev.azure.com/${context.organisationName}/${context.projectName}/_workitems/edit/${workItemId}`;
  }

  private formatAdoFilePath(
    repositoryRoot: string,
    filePath: string,
    workingDirectory: string,
  ): string {
    const canonical = path.resolve(workingDirectory, filePath);
    const relative = path.relative(repositoryRoot, canonical);

    if (!relative || relative.startsWith('..')) {
      throw new CliError('File path must be inside the git repository root.');
    }

    const normalized = `/${relative.split(path.sep).join('/')}`;
    return normalized;
  }
}
