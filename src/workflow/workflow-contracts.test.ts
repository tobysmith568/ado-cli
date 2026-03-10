import { describe, expect, it } from 'bun:test';

async function readWorkflow(fileName: string): Promise<string> {
  return Bun.file(`.github/workflows/${fileName}`).text();
}

describe('Workflow contracts', () => {
  it('deployment consumes the same artifact names produced by integration', async () => {
    const integration = await readWorkflow('integration.yml');
    const deployment = await readWorkflow('deployment.yml');

    expect(integration).toContain('name: ado-${{ matrix.os }}');

    const artifactNames = ['ado-ubuntu-latest', 'ado-macos-latest', 'ado-windows-latest'];

    for (const name of artifactNames) {
      expect(deployment).toContain(`name: ${name}`);
    }
  });

  it('deployment tags releases with a v-prefixed version input', async () => {
    const deployment = await readWorkflow('deployment.yml');

    expect(deployment).toContain('tag_name: v${{ inputs.version }}');
  });

  it('deployment reuses integration workflow before release job', async () => {
    const deployment = await readWorkflow('deployment.yml');

    expect(deployment).toContain('uses: ./.github/workflows/integration.yml');
    expect(deployment).toContain('needs:');
    expect(deployment).toContain('create-release');
  });
});
