import { mock as bunMock, describe, expect, it } from 'bun:test';
import { createMock } from 'mock-extended';
import type { BrowserService } from '../services/browser-service';
import type { RepositoryService } from '../services/repository-service';
import { FilesCommand } from './files-command';

describe('FilesCommand', () => {
  const mock = createMock(() => bunMock());

  it('opens branch files URL when file path is not provided', async () => {
    const repositoryService = mock<RepositoryService>();
    const browserService = mock<BrowserService>();

    const repository = {
      organisationName: 'org',
      projectName: 'project',
      repositoryName: 'repo',
      localPath: '/repo',
    };

    repositoryService.parseFromDirectory.mockReturnValue(repository);
    repositoryService.resolveBranchName.mockResolvedValue('feature/abc');
    repositoryService.getFilesUrlForBranch.mockReturnValue('https://example.dev/files');

    const command = new FilesCommand(
      repositoryService as unknown as RepositoryService,
      browserService as unknown as BrowserService,
    );

    await command.execute({ directory: '/repo' });

    expect(repositoryService.getFilesUrlForBranch).toHaveBeenCalledWith(repository, 'feature/abc');
    expect(browserService.open).toHaveBeenCalledWith('https://example.dev/files');
  });

  it('opens file URL when file path is provided', async () => {
    const repositoryService = mock<RepositoryService>();
    const browserService = mock<BrowserService>();

    const repository = {
      organisationName: 'org',
      projectName: 'project',
      repositoryName: 'repo',
      localPath: '/repo',
    };

    repositoryService.parseFromDirectory.mockReturnValue(repository);
    repositoryService.resolveBranchName.mockResolvedValue('main');
    repositoryService.getFilesUrlForFileOnBranch.mockReturnValue('https://example.dev/file');

    const command = new FilesCommand(
      repositoryService as unknown as RepositoryService,
      browserService as unknown as BrowserService,
    );

    await command.execute({ directory: '/repo', filePath: 'src/main.ts' });

    expect(repositoryService.getFilesUrlForFileOnBranch).toHaveBeenCalledWith(
      repository,
      'main',
      'src/main.ts',
      '/repo',
    );
    expect(browserService.open).toHaveBeenCalledWith('https://example.dev/file');
  });
});
