import * as path from 'path';
import {
    debug,
    window,
    DebugAdapterDescriptorFactory,
    DebugSession,
    DebugAdapterExecutable,
    DebugAdapterDescriptor,
    ExtensionContext,
    OutputChannel,
    ProviderResult,
  } from 'vscode';
import * as os from 'os';

let outputChannel: OutputChannel;

export function registerDebugger(context: ExtensionContext) {
  outputChannel = window.createOutputChannel('SolidityDebugger');

  context.subscriptions.push(
    debug.registerDebugAdapterDescriptorFactory('solidity', new SolidityDebugAdapterDescriptorFactory(context)),
    debug.onDidTerminateDebugSession(() => {
      outputChannel.appendLine(`Debug session ended.`);
    }),
  );
}

export class SolidityDebugAdapterDescriptorFactory implements DebugAdapterDescriptorFactory {
  context: ExtensionContext;

  constructor(context: ExtensionContext) {
    this.context = context;
  }

  async createDebugAdapterDescriptor(
    _session: DebugSession,
    _executable: DebugAdapterExecutable,
  ): Promise<ProviderResult<DebugAdapterDescriptor>> {
    const serverBinary = this.context.asAbsolutePath(
      path.join('dist',
      os.platform().startsWith("win") ? 'foundry-dap-server.exe' : 'foundry-dap-server')
    );

    if (!serverBinary) {
      throw new Error('Could not find Solidity debugger server');
    }

    return new DebugAdapterExecutable(serverBinary, []);
  }
}