import open from 'open';

export class BrowserService {
  public async open(url: string): Promise<void> {
    await open(url);
  }
}
