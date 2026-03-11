#!/usr/bin/env bun

import { Command } from "@commander-js/extra-typings";
import { buildRootContainer } from "../composition/root-factory";
import { CliError } from "./cli-error";

async function run(): Promise<void> {
  const withContainer = async (
    callback: (
      container: Awaited<ReturnType<typeof buildRootContainer>>,
    ) => Promise<void>,
  ): Promise<void> => {
    const container = await buildRootContainer();
    await callback(container);
  };

  const withOptional = <T extends Record<string, unknown>>(
    key: string,
    value: string | boolean | undefined,
  ): Partial<T> => {
    if (value === undefined) {
      return {};
    }

    return { [key]: value } as Partial<T>;
  };

  const program = new Command("ado")
    .description(
      "Open browser tabs to Azure DevOps pages for the current repository",
    )
    .showHelpAfterError();

  program
    .command("files")
    .alias("file")
    .description("Opens the file explorer page for the current branch")
    .option("-d, --directory <directory>", "Directory to run the command from")
    .option("-b, --branch <branch>", "Branch name to use")
    .argument("[filePath]", "Optional path to a file or directory to show")
    .action(async (filePath, options) => {
      await withContainer(async (container) => {
        await container.filesCommand.execute({
          ...withOptional("directory", options.directory),
          ...withOptional("branch", options.branch),
          ...withOptional("filePath", filePath),
        });
      });
    });

  program
    .command("pr")
    .description("Open existing PR for branch or prompt to create one")
    .option("-d, --directory <directory>", "Directory to run the command from")
    .option("-b, --branch <branch>", "Branch name to use")
    .option("-c, --create", "Always create a new pull request")
    .action(async (options) => {
      await withContainer(async (container) => {
        await container.prCommand.execute({
          ...withOptional("directory", options.directory),
          ...withOptional("branch", options.branch),
          ...withOptional("create", options.create),
        });
      });
    });

  program
    .command("item")
    .alias("pbi")
    .alias("bug")
    .description("Open linked work item for current branch PR")
    .option("-d, --directory <directory>", "Directory to run the command from")
    .option("-b, --branch <branch>", "Branch name to use")
    .option("--id <id>", "Work item ID to open directly")
    .action(async (options) => {
      await withContainer(async (container) => {
        await container.itemCommand.execute({
          ...withOptional("directory", options.directory),
          ...withOptional("branch", options.branch),
          ...withOptional("id", options.id),
        });
      });
    });

  await program.parseAsync(process.argv);
}

run().catch((error: unknown) => {
  if (error instanceof CliError) {
    console.error(error.message);
    process.exit(1);
  }

  console.error(error);
  process.exit(1);
});
