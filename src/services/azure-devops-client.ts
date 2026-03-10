import * as azdev from 'azure-devops-node-api';
import type { IGitApi } from 'azure-devops-node-api/GitApi';
import * as GitInterfaces from 'azure-devops-node-api/interfaces/GitInterfaces.js';
import { CliError } from '../cli/cli-error';

export class AzureDevOpsClient {
  public constructor(private readonly pat: string) {}

  public async getRepositoryId(
    organisationName: string,
    projectName: string,
    repositoryName: string,
  ): Promise<string> {
    try {
      const gitApi = await this.getGitApi(organisationName);

      const repository = await gitApi.getRepository(repositoryName, projectName);
      const repositoryId = repository.id;

      if (!repositoryId) {
        throw new CliError('Unable to resolve repository ID from Azure DevOps API.');
      }

      return repositoryId;
    } catch (error) {
      throw this.mapApiError('resolve repository ID', error);
    }
  }

  public async getOpenPrIdForBranch(
    organisationName: string,
    projectName: string,
    repositoryId: string,
    branchName: string,
  ): Promise<number | undefined> {
    try {
      const gitApi = await this.getGitApi(organisationName);
      const sourceRefName = this.toFullRef(branchName);

      const criteria: GitInterfaces.GitPullRequestSearchCriteria = {
        status: GitInterfaces.PullRequestStatus.Active,
        sourceRefName,
      };

      const prs = await gitApi.getPullRequests(repositoryId, criteria, projectName);
      return prs[0]?.pullRequestId ?? undefined;
    } catch (error) {
      throw this.mapApiError('resolve pull request for branch', error);
    }
  }

  public async getPullRequestWorkItemIds(
    organisationName: string,
    projectName: string,
    repositoryName: string,
    pullRequestId: number,
  ): Promise<string[]> {
    try {
      const gitApi = await this.getGitApi(organisationName);

      const workItems = await gitApi.getPullRequestWorkItemRefs(
        repositoryName,
        pullRequestId,
        projectName,
      );

      return workItems
        .map((workItem: { id?: string }) => workItem.id)
        .filter((value: string | undefined): value is string => Boolean(value));
    } catch (error) {
      throw this.mapApiError('resolve pull request work items', error);
    }
  }

  private async getGitApi(organisationName: string): Promise<IGitApi> {
    try {
      const authHandler = azdev.getPersonalAccessTokenHandler(this.pat);
      const connection = new azdev.WebApi(`https://dev.azure.com/${organisationName}`, authHandler);
      return await connection.getGitApi();
    } catch (error) {
      throw this.mapApiError('create Azure DevOps Git API client', error);
    }
  }

  private mapApiError(operation: string, error: unknown): CliError {
    if (error instanceof CliError) {
      return error;
    }

    const message = String(error);

    if (this.isAuthError(message)) {
      return new CliError(
        `Azure DevOps authentication failed while trying to ${operation}. Check your PAT and try again.`,
      );
    }

    if (this.isNotFoundError(message)) {
      return new CliError(
        `Azure DevOps resource not found while trying to ${operation}. Check your organisation/project/repository values.`,
      );
    }

    return new CliError(`Invalid response from Azure Devops API while trying to ${operation}.`);
  }

  private isAuthError(message: string): boolean {
    return /\b401\b|\b403\b|unauthori[sz]ed|forbidden/i.test(message);
  }

  private isNotFoundError(message: string): boolean {
    return /\b404\b|not found/i.test(message);
  }

  private toFullRef(branchName: string): string {
    return branchName.startsWith('refs/heads/') ? branchName : `refs/heads/${branchName}`;
  }
}
