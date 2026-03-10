import type { BrowserService } from '../services/browser-service';
import type { RepositoryService } from '../services/repository-service';

export type FilesCommandArgs = {
  directory?: string;
  branch?: string;
  filePath?: string;
};

export class FilesCommand {
  public constructor(
    private readonly repositoryService: RepositoryService,
    private readonly browserService: BrowserService,
  ) {}

  public async execute(args: FilesCommandArgs): Promise<void> {
    const workingDirectory = args.directory ?? process.cwd();
    const repository = this.repositoryService.parseFromDirectory(workingDirectory);
    const branchName = await this.repositoryService.resolveBranchName(
      repository.localPath,
      args.branch,
    );

    const url = args.filePath
      ? this.repositoryService.getFilesUrlForFileOnBranch(
          repository,
          branchName,
          args.filePath,
          workingDirectory,
        )
      : this.repositoryService.getFilesUrlForBranch(repository, branchName);

    await this.browserService.open(url);
  }
}
