import { workspace, ExtensionContext } from 'vscode';
import {
	LanguageClient,
} from 'vscode-languageclient/node';
import { createLinterClient } from './linter';
import { createFoundryCompilerClient } from './foundry-compiler';
import { createSlitherClient } from './slither';
import { createTestsPositionsClient } from './tests-positions';
import registerForgeFmtLinter from "./fmt-wrapper";
import { TestManager } from './tests/test-manager';

let slitherClient: LanguageClient;
let linterClient: LanguageClient;
let foundryCompilerClient: LanguageClient;
let testsPositionsClient: LanguageClient;
let testManager: TestManager;

export async function activate(context: ExtensionContext) {
	linterClient = createLinterClient(context);
	foundryCompilerClient = createFoundryCompilerClient(context);
	slitherClient = createSlitherClient(context);
	testsPositionsClient = await createTestsPositionsClient(context);
	if (vscode.workspace.workspaceFolders?.length)
		testManager = new TestManager(testsPositionsClient, vscode.workspace.workspaceFolders[0].uri.fsPath);

	context.subscriptions.push(linterClient, foundryCompilerClient, slitherClient, testsPositionsClient, testManager.testController);

	registerForgeFmtLinter(context);

	
	const folders = workspace.workspaceFolders;
	if (folders) {
		const files = await vscode.workspace.findFiles('**/*.sol', `${folders[0].uri.fsPath}/**`);
		files.forEach(file => {
			if (!file.path.includes('forge-std')) {
				workspace.openTextDocument(file);
			}
		});
	}

}
