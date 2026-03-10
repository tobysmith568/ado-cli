import { CliError } from '../cli/cli-error';
import type { InkPrompts } from '../ui/ink-prompts';
import type { ConfigStore } from './config-store';

type StorageChoice = 'pat' | 'env_var';

export class ApiKeyManager {
  public constructor(
    private readonly configStore: ConfigStore,
    private readonly prompts: InkPrompts,
  ) {}

  public async getApiKey(): Promise<string> {
    const current = this.tryGetCurrentPat();

    if (current) {
      return current;
    }

    return this.setAndGetNewPat();
  }

  private tryGetCurrentPat(): string | undefined {
    const pat = this.configStore.readValue('credential', 'pat');
    if (pat) {
      return pat;
    }

    const envVarName = this.configStore.readValue('credential', 'env_var');
    if (!envVarName) {
      return undefined;
    }

    return process.env[envVarName];
  }

  private async setAndGetNewPat(): Promise<string> {
    const storageChoice = await this.prompts.select<StorageChoice>(
      'How do you want to store your ADO PAT?',
      [
        { label: 'Store PAT in ~/.ado_cli', value: 'pat' },
        { label: 'Store env var name in ~/.ado_cli', value: 'env_var' },
      ],
    );

    if (storageChoice === 'pat') {
      const pat = await this.prompts.text('Please paste in your ADO PAT:', { mask: true });
      this.configStore.setValue('credential', 'pat', pat);
      return pat;
    }

    const envVarName = await this.prompts.text(
      'Please enter the name of your Environment Variable:',
    );
    const envVarValue = process.env[envVarName];

    if (!envVarValue) {
      throw new CliError(`Environment variable ${envVarName} was not found or is empty.`);
    }

    this.configStore.setValue('credential', 'env_var', envVarName);
    return envVarValue;
  }
}
