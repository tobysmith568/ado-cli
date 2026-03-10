import { describe, expect, it } from 'bun:test';
import { InkPrompts } from './ink-prompts';

describe('InkPrompts', () => {
  it('returns false for confirm in non-interactive mode', async () => {
    const originalIsTTY = process.stdin.isTTY;
    Object.defineProperty(process.stdin, 'isTTY', { value: false, configurable: true });

    try {
      const prompts = new InkPrompts();
      const result = await prompts.confirm('Should create PR?');
      expect(result).toBe(false);
    } finally {
      Object.defineProperty(process.stdin, 'isTTY', {
        value: originalIsTTY,
        configurable: true,
      });
    }
  });

  it('returns first option for select in non-interactive mode', async () => {
    const originalIsTTY = process.stdin.isTTY;
    Object.defineProperty(process.stdin, 'isTTY', { value: false, configurable: true });

    try {
      const prompts = new InkPrompts();
      const result = await prompts.select('Storage mode?', [
        { label: 'Direct PAT', value: 'pat' as const },
        { label: 'Environment Variable', value: 'env_var' as const },
      ]);

      expect(result).toBe('pat');
    } finally {
      Object.defineProperty(process.stdin, 'isTTY', {
        value: originalIsTTY,
        configurable: true,
      });
    }
  });
});
