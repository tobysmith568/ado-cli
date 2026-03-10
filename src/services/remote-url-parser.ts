import { CliError } from "../cli/cli-error";
import type { ParsedRemoteUrl } from "../domain/remote-url";

const HTTP_WITH_USER =
  /^https:\/\/.*?@dev\.azure\.com\/(.*?)\/(.*?)\/_git\/(.*?)$/;
const HTTP_NO_USER = /^https:\/\/dev\.azure\.com\/(.*?)\/(.*?)\/_git\/(.*?)$/;
const SSH = /^git@ssh\.dev\.azure\.com:v\d\/(.*?)\/(.*?)\/(.*?)$/;

export class RemoteUrlParser {
  public parse(remoteUrl: string): ParsedRemoteUrl {
    if (remoteUrl.startsWith("http")) {
      return this.parseHttp(remoteUrl);
    }

    return this.parseSsh(remoteUrl);
  }

  private parseHttp(remoteUrl: string): ParsedRemoteUrl {
    const match =
      HTTP_WITH_USER.exec(remoteUrl) ?? HTTP_NO_USER.exec(remoteUrl);

    if (!match) {
      throw new CliError("Cannot parse remote url");
    }

    const organisationName = match[1];
    const projectName = match[2];
    const repositoryName = match[3];

    if (!organisationName || !projectName || !repositoryName) {
      throw new CliError("Cannot parse remote url");
    }

    return { organisationName, projectName, repositoryName };
  }

  private parseSsh(remoteUrl: string): ParsedRemoteUrl {
    const match = SSH.exec(remoteUrl);

    if (!match) {
      throw new CliError("Cannot parse remote url");
    }

    const organisationName = match[1];
    const projectName = match[2];
    const repositoryName = match[3];

    if (!organisationName || !projectName || !repositoryName) {
      throw new CliError("Cannot parse remote url");
    }

    return { organisationName, projectName, repositoryName };
  }
}
