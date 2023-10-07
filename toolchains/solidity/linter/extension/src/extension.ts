// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import { workspace, ExtensionContext, window,  commands } from 'vscode';
import { LanguageClient, LanguageClientOptions, ServerOptions, TransportKind } from 'vscode-languageclient/node';
import * as path from 'path';

let client: LanguageClient;

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed
export async function activate(context: ExtensionContext) {
	const binaryPath = path.join(context.extensionPath, 'core');
	console.log(binaryPath);

	const workspacePath = workspace.workspaceFolders?.[0].uri.fsPath!;
	const configPath = path.join(workspacePath, '.solidhunter.json');

	if (!workspacePath) {
		return;
	}

	const serverOptions: ServerOptions = {
		run: { command: binaryPath, args: ['-p', workspacePath, '-r', configPath], transport: TransportKind.stdio },
		debug: { command: binaryPath, args: ['-p', workspacePath, '-r', configPath], transport: TransportKind.stdio }
	};
	
	const traceOutputChannel = window.createOutputChannel("Osmium Solidity Linter Trace");
	// Options to control the language client
	const clientOptions: LanguageClientOptions = {
		// Register the server for plain text documents
		documentSelector: [
			{scheme: "file", language:'sol'}
		],
		synchronize: {
			// Notify the server about file changes to '.clientrc files contained in the workspace
			fileEvents: workspace.createFileSystemWatcher('**/.clientrc')
		},
		traceOutputChannel
	};

	// Create the language client and start the client.
	client = new LanguageClient(
		'osmium-solidity-linter',
		'Osmium Solidity Linter',
		serverOptions,
		clientOptions
	);

	// Start the client. This will also launch the server
	await client.start();
}

// This method is called when your extension is deactivated
export function deactivate() {
	if (!client) {
		return undefined;
	}
	return client.stop();
}