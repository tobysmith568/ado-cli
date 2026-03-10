import { describe, expect, it } from 'bun:test';
import { CliError } from '../cli/cli-error';
import { AzureDevOpsClient } from './azure-devops-client';

describe('AzureDevOpsClient', () => {
  it('normalizes plain branch names to full refs for PR lookup', async () => {
    const client = new AzureDevOpsClient('pat') as unknown as {
      getOpenPrIdForBranch: AzureDevOpsClient['getOpenPrIdForBranch'];
      getGitApi: () => Promise<{
        getPullRequests: (
          repositoryId: string,
          criteria: { sourceRefName?: string },
          projectName: string,
        ) => Promise<Array<{ pullRequestId?: number }>>;
      }>;
    };

    let capturedSourceRef = '';

    client.getGitApi = async () => ({
      getPullRequests: async (_repositoryId, criteria, _projectName) => {
        capturedSourceRef = criteria.sourceRefName ?? '';
        return [{ pullRequestId: 7 }];
      },
    });

    const prId = await client.getOpenPrIdForBranch('org', 'project', 'repo-id', 'feature/demo');

    expect(prId).toBe(7);
    expect(capturedSourceRef).toBe('refs/heads/feature/demo');
  });

  it('maps auth failures to a clear CliError message', async () => {
    const client = new AzureDevOpsClient('bad-pat') as unknown as {
      getRepositoryId: AzureDevOpsClient['getRepositoryId'];
      getGitApi: () => Promise<{
        getRepository: () => Promise<{ id?: string }>;
      }>;
    };

    client.getGitApi = async () => ({
      getRepository: async () => {
        throw new Error('401 Unauthorized');
      },
    });

    await expect(client.getRepositoryId('org', 'project', 'repo')).rejects.toBeInstanceOf(CliError);
    await expect(client.getRepositoryId('org', 'project', 'repo')).rejects.toThrow(
      'authentication failed',
    );
  });

  it('maps not-found failures to a clear CliError message', async () => {
    const client = new AzureDevOpsClient('pat') as unknown as {
      getPullRequestWorkItemIds: AzureDevOpsClient['getPullRequestWorkItemIds'];
      getGitApi: () => Promise<{
        getPullRequestWorkItemRefs: () => Promise<Array<{ id?: string }>>;
      }>;
    };

    client.getGitApi = async () => ({
      getPullRequestWorkItemRefs: async () => {
        throw new Error('404 Not Found');
      },
    });

    await expect(
      client.getPullRequestWorkItemIds('org', 'project', 'repo', 1),
    ).rejects.toBeInstanceOf(CliError);
    await expect(client.getPullRequestWorkItemIds('org', 'project', 'repo', 1)).rejects.toThrow(
      'resource not found',
    );
  });
});
