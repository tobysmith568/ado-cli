import { mock as bunMock, describe, expect, it } from "bun:test";
import { createMock } from "mock-extended";
import { CliError } from "../cli/cli-error";
import type { InkPrompts } from "../ui/ink-prompts";
import { ApiKeyManager } from "./api-key-manager";
import type { ConfigStore } from "./config-store";

describe("ApiKeyManager", () => {
  const mock = createMock(() => bunMock());

  it("returns PAT from config without prompting", async () => {
    const configStore = mock<ConfigStore>();
    const prompts = mock<InkPrompts>();

    configStore.readValue.mockImplementation((section, key) =>
      section === "credential" && key === "pat" ? "from-config" : undefined,
    );

    const manager = new ApiKeyManager(
      configStore as unknown as ConfigStore,
      prompts,
    );
    const result = await manager.getApiKey();

    expect(result).toBe("from-config");
    expect(prompts.select).not.toHaveBeenCalled();
  });

  it("returns PAT from env var when env var name is configured", async () => {
    const original = process.env.ADO_TEST_PAT;
    process.env.ADO_TEST_PAT = "from-env";

    try {
      const configStore = mock<ConfigStore>();
      const prompts = mock<InkPrompts>();

      configStore.readValue.mockImplementation((section, key) =>
        section === "credential" && key === "env_var"
          ? "ADO_TEST_PAT"
          : undefined,
      );

      const manager = new ApiKeyManager(
        configStore as unknown as ConfigStore,
        prompts,
      );
      const result = await manager.getApiKey();

      expect(result).toBe("from-env");
      expect(prompts.select).not.toHaveBeenCalled();
    } finally {
      if (original === undefined) {
        process.env.ADO_TEST_PAT = undefined;
      } else {
        process.env.ADO_TEST_PAT = original;
      }
    }
  });

  it("stores PAT when PAT storage mode is selected", async () => {
    const configStore = mock<ConfigStore>();
    const prompts = mock<InkPrompts>();

    configStore.readValue.mockReturnValue(undefined);
    prompts.select.mockResolvedValue("pat");
    prompts.text.mockResolvedValue("new-pat-value");

    const manager = new ApiKeyManager(
      configStore as unknown as ConfigStore,
      prompts,
    );
    const result = await manager.getApiKey();

    expect(result).toBe("new-pat-value");
    expect(configStore.setValue).toHaveBeenCalledWith(
      "credential",
      "pat",
      "new-pat-value",
    );
  });

  it("throws when env var mode is selected and variable is missing", async () => {
    process.env.ADO_MISSING_PAT = undefined;

    const configStore = mock<ConfigStore>();
    const prompts = mock<InkPrompts>();

    configStore.readValue.mockReturnValue(undefined);
    prompts.select.mockResolvedValue("env_var");
    prompts.text.mockResolvedValue("ADO_MISSING_PAT");

    const manager = new ApiKeyManager(
      configStore as unknown as ConfigStore,
      prompts,
    );

    await expect(manager.getApiKey()).rejects.toBeInstanceOf(CliError);
  });
});
