import { describe, expect, it } from 'bun:test';
import { CliError } from '../cli/cli-error';
import type { InkPrompts } from '../ui/ink-prompts';
import { ApiKeyManager } from './api-key-manager';
import type { ConfigStore } from './config-store';

describe('ApiKeyManager', () => {
  it('returns PAT from config without prompting', async () => {
    const configStore = {
      readValue: (section: string, key: string) =>
        section === 'credential' && key === 'pat' ? 'from-config' : undefined,
      setValue: () => {},
    } as unknown as ConfigStore;

    const prompts = {
      select: async () => 'pat',
      text: async () => 'unused',
    } as unknown as InkPrompts;

    const manager = new ApiKeyManager(configStore, prompts);
    const result = await manager.getApiKey();

    expect(result).toBe('from-config');
  });

  it('returns PAT from env var when env var name is configured', async () => {
    const original = process.env.ADO_TEST_PAT;
    process.env.ADO_TEST_PAT = 'from-env';

    try {
      const configStore = {
        readValue: (section: string, key: string) =>
          section === 'credential' && key === 'env_var' ? 'ADO_TEST_PAT' : undefined,
        setValue: () => {},
      } as unknown as ConfigStore;

      const prompts = {
        select: async () => 'pat',
        text: async () => 'unused',
      } as unknown as InkPrompts;

      const manager = new ApiKeyManager(configStore, prompts);
      const result = await manager.getApiKey();

      expect(result).toBe('from-env');
    } finally {
      if (original === undefined) {
        process.env.ADO_TEST_PAT = undefined;
      } else {
        process.env.ADO_TEST_PAT = original;
      }
    }
  });

  it('stores PAT when PAT storage mode is selected', async () => {
    const writes: Array<{ section: string; key: string; value: string }> = [];

    const configStore = {
      readValue: () => undefined,
      setValue: (section: string, key: string, value: string) => {
        writes.push({ section, key, value });
      },
    } as unknown as ConfigStore;

    const prompts = {
      select: async () => 'pat' as const,
      text: async () => 'new-pat-value',
    } as unknown as InkPrompts;

    const manager = new ApiKeyManager(configStore, prompts);
    const result = await manager.getApiKey();

    expect(result).toBe('new-pat-value');
    expect(writes).toEqual([{ section: 'credential', key: 'pat', value: 'new-pat-value' }]);
  });

  it('throws when env var mode is selected and variable is missing', async () => {
    process.env.ADO_MISSING_PAT = undefined;

    const configStore = {
      readValue: () => undefined,
      setValue: () => {},
    } as unknown as ConfigStore;

    const prompts = {
      select: async () => 'env_var' as const,
      text: async () => 'ADO_MISSING_PAT',
    } as unknown as InkPrompts;

    const manager = new ApiKeyManager(configStore, prompts);

    await expect(manager.getApiKey()).rejects.toBeInstanceOf(CliError);
  });
});
