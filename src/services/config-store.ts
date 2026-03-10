import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import ini from 'ini';

export class ConfigStore {
  private readonly configPath: string;

  public constructor(configPath?: string) {
    this.configPath = configPath ?? path.join(os.homedir(), '.ado_cli');
  }

  public readValue(section: string, key: string): string | undefined {
    const data = this.readConfig();
    const sectionValue = data[section] as Record<string, unknown> | undefined;
    const value = sectionValue?.[key];

    return typeof value === 'string' ? value : undefined;
  }

  public setValue(section: string, key: string, value: string): void {
    const data = this.readConfig();
    const sectionValue = (data[section] as Record<string, unknown> | undefined) ?? {};
    sectionValue[key] = value;
    data[section] = sectionValue;
    this.writeConfig(data);
  }

  private readConfig(): Record<string, unknown> {
    this.ensureFileExists();
    const content = readFileSync(this.configPath, 'utf8');
    const parsed = ini.parse(content);

    return parsed as Record<string, unknown>;
  }

  private writeConfig(data: Record<string, unknown>): void {
    this.ensureFileExists();
    writeFileSync(this.configPath, ini.stringify(data), { mode: 0o600 });
  }

  private ensureFileExists(): void {
    const directory = path.dirname(this.configPath);

    if (!existsSync(directory)) {
      mkdirSync(directory, { recursive: true });
    }

    if (!existsSync(this.configPath)) {
      writeFileSync(this.configPath, '', { mode: 0o600 });
    }
  }
}
