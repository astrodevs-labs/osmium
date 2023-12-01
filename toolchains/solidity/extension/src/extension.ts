/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import * as path from 'path';
import { workspace, ExtensionContext, Uri } from 'vscode';
import { TextDecoder } from 'util';

import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient;

export async function activate(context: ExtensionContext) {
	// The server is implemented in node
	const serverBinary = context.asAbsolutePath(
		path.join('dist', 'linter-server')
	);

	// If the extension is launched in debug mode then the debug server options are used
	// Otherwise the run options are used
	const serverOptions: ServerOptions = {
		run: { command: serverBinary, transport: TransportKind.stdio },
		debug: {
			command: serverBinary,
			transport: TransportKind.stdio,
		}
	};

	// Options to control the language client
	const clientOptions: LanguageClientOptions = {
		// Register the server for plain text documents
		documentSelector: [{ scheme: 'file', language: 'solidity' }],
		synchronize: {
			// Notify the server about file changes to '.clientrc files contained in the workspace
			fileEvents: workspace.createFileSystemWatcher('**/.solidhunter.json')
		}
	};

	// Create the language client and start the client.
	client = new LanguageClient(
		'osmium-solidity',
		'Osmium Solidity Language Server',
		serverOptions,
		clientOptions
	);

	client.onRequest('osmium/getContent', async (params) => {
    	const contentUint8 = await workspace.fs.readFile(Uri.parse(params.uri));
    	const content = new TextDecoder().decode(contentUint8);
    	return content;
	});

	// Start the client. This will also launch the server
	client.start();

	const folders = workspace.workspaceFolders;
	if (folders) {
		const files = await workspace.findFiles('**/*.sol', `${folders[0].uri.fsPath}/**`);
		files.forEach(file => {
			workspace.openTextDocument(file);
		});
	}

}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}
