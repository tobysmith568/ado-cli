import type { ParsedRemoteUrl } from "./remote-url";

export type RepositoryContext = ParsedRemoteUrl & {
  localPath: string;
};
