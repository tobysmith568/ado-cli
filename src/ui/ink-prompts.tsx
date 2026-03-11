import { Box, render, Text, useApp, useInput } from "ink";
import { useState } from "react";
import { CliError } from "../cli/cli-error";

type SelectOption<T extends string> = {
  label: string;
  value: T;
};

type SelectPromptProps<T extends string> = {
  question: string;
  options: SelectOption<T>[];
  onSubmit: (value: T) => void;
};

function SelectPrompt<T extends string>({
  question,
  options,
  onSubmit,
}: SelectPromptProps<T>) {
  const [index, setIndex] = useState(0);
  const { exit } = useApp();
  const firstOption = options[0];

  useInput((input, key) => {
    if (key.upArrow) {
      setIndex((current) => (current - 1 + options.length) % options.length);
      return;
    }

    if (key.downArrow) {
      setIndex((current) => (current + 1) % options.length);
      return;
    }

    if (key.return) {
      const selectedOption = options[index];
      if (!selectedOption) {
        throw new CliError("No option selected.");
      }

      onSubmit(selectedOption.value);
      exit();
      return;
    }

    if (input === "j") {
      setIndex((current) => (current + 1) % options.length);
    }

    if (input === "k") {
      setIndex((current) => (current - 1 + options.length) % options.length);
    }
  });

  if (!firstOption) {
    return (
      <Box flexDirection="column">
        <Text>No options available.</Text>
      </Box>
    );
  }

  return (
    <Box flexDirection="column">
      <Text>{question}</Text>
      {options.map((option, optionIndex) => {
        const selected = optionIndex === index;

        return (
          <Text
            key={option.value}
            {...(selected ? { color: "cyan" as const } : {})}
          >
            {selected ? "> " : "  "}
            {option.label}
          </Text>
        );
      })}
      <Text dimColor>Use up/down arrows then Enter.</Text>
    </Box>
  );
}

type TextPromptProps = {
  question: string;
  mask?: boolean;
  onSubmit: (value: string) => void;
};

function TextPrompt({ question, mask = false, onSubmit }: TextPromptProps) {
  const [value, setValue] = useState("");
  const { exit } = useApp();

  useInput((input, key) => {
    if (key.return) {
      onSubmit(value.trim());
      exit();
      return;
    }

    if (key.backspace || key.delete) {
      setValue((current) => current.slice(0, -1));
      return;
    }

    if (input.length > 0 && !key.ctrl && !key.meta) {
      setValue((current) => `${current}${input}`);
    }
  });

  return (
    <Box flexDirection="column">
      <Text>{question}</Text>
      <Text color="cyan">{mask ? "*".repeat(value.length) : value}</Text>
      <Text dimColor>Type and press Enter.</Text>
    </Box>
  );
}

export class InkPrompts {
  public async select<T extends string>(
    question: string,
    options: SelectOption<T>[],
  ): Promise<T> {
    if (options.length === 0) {
      throw new CliError("Prompt options are required.");
    }

    const firstOption = options[0];

    if (!firstOption) {
      throw new CliError("Prompt options are required.");
    }

    if (!process.stdin.isTTY) {
      return firstOption.value;
    }

    return new Promise<T>((resolve) => {
      render(
        <SelectPrompt
          question={question}
          options={options}
          onSubmit={resolve}
        />,
      );
    });
  }

  public async text(
    question: string,
    options?: { mask?: boolean },
  ): Promise<string> {
    if (!process.stdin.isTTY) {
      throw new CliError(
        "Interactive prompt requested in non-interactive mode.",
      );
    }

    return new Promise<string>((resolve) => {
      render(
        <TextPrompt
          question={question}
          mask={options?.mask ?? false}
          onSubmit={resolve}
        />,
      );
    });
  }

  public async confirm(question: string): Promise<boolean> {
    if (!process.stdin.isTTY) {
      return false;
    }

    const choice = await this.select(question, [
      { label: "Yes", value: "yes" },
      { label: "No", value: "no" },
    ]);

    return choice === "yes";
  }
}
