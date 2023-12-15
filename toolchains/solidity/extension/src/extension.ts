/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import * as path from 'path';
import { workspace, ExtensionContext } from 'vscode';
import {
	LanguageClient,
} from 'vscode-languageclient/node';
import { createLinterClient } from './linter';
import { createFoundryCompilerClient } from './foundry-compiler';
import { createTestsPositionsClient } from './tests-positions';
import { TestManager } from './tests/test-manager';

let linterClient: LanguageClient;
let foundryCompilerClient: LanguageClient;
let testsPositionsClient: LanguageClient;
let testManager: TestManager;

export async function activate(context: ExtensionContext) {
	linterClient = await createLinterClient(context);
	foundryCompilerClient = createFoundryCompilerClient(context);
	testsPositionsClient = await createTestsPositionsClient(context);
	if (workspace.workspaceFolders?.length)
		testManager = new TestManager(testsPositionsClient, workspace.workspaceFolders[0].uri.fsPath);

	// Push the disposable to the context's subscriptions so that the
	// client can be deactivated on extension deactivation
	context.subscriptions.push(linterClient, foundryCompilerClient, testsPositionsClient, testManager.testController);

	
	const folders = workspace.workspaceFolders;
	if (folders) {
		const files = await workspace.findFiles('**/*.sol', `${folders[0].uri.fsPath}/**`);
		files.forEach(file => {
			if (!file.path.includes('forge-std'))
				workspace.openTextDocument(file);
		});
	}

}
