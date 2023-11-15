/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';

import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	TransportKind
} from 'vscode-languageclient/node';
import { createLinterClient } from './linter';
import { createFoundryCompilerClient } from './foundry-compiler';

let linterClient: LanguageClient;
let foundryCompilerClient: LanguageClient;

let linterClient: LanguageClient;

export async function activate(context: ExtensionContext) {
	linterClient = createLinterClient(context);
	foundryCompilerClient = createFoundryCompilerClient(context);

	context.subscriptions.push(linterClient);
	context.subscriptions.push(foundryCompilerClient);

	
	const folders = workspace.workspaceFolders;
	if (folders) {
		const files = await workspace.findFiles('**/*.sol', `${folders[0].uri.fsPath}/**`);
		files.forEach(file => {
			workspace.openTextDocument(file);
		});
	}

}
