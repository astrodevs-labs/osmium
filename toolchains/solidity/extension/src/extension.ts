import { workspace, ExtensionContext, window } from 'vscode';
import {
	LanguageClient,
} from 'vscode-languageclient/node';
import { createLinterClient } from './linter';
import { createFoundryCompilerClient } from './foundry-compiler';
import { createSlitherClient } from './slither';
import { createTestsPositionsClient } from './tests-positions';
import registerForgeFmtLinter from "./fmt-wrapper";
import { TestManager } from './tests/test-manager';
import { SidePanelProvider } from './sidepanel-provider';

/*let slitherClient: LanguageClient;
let linterClient: LanguageClient;
let foundryCompilerClient: LanguageClient;
let testsPositionsClient: LanguageClient;
let testManager: TestManager;*/

export async function activate(context: ExtensionContext) {
	/*linterClient = await createLinterClient(context);
	foundryCompilerClient = createFoundryCompilerClient(context);
	slitherClient = createSlitherClient(context);
	testsPositionsClient = await createTestsPositionsClient(context);
	if (workspace.workspaceFolders?.length)
		testManager = new TestManager(testsPositionsClient, workspace.workspaceFolders[0].uri.fsPath);

	context.subscriptions.push(linterClient, foundryCompilerClient, slitherClient, testsPositionsClient, testManager.testController);

	

	registerForgeFmtLinter(context);*/

	
	const folders = workspace.workspaceFolders;
	if (folders) {
		const files = await workspace.findFiles('**/*.sol', `${folders[0].uri.fsPath}/**`);
		files.forEach(file => {
			if (!file.path.includes('forge-std')) {
				workspace.openTextDocument(file);
			}
		});
	}

	const provider = new SidePanelProvider(context.extensionUri);

	context.subscriptions.push(window.registerWebviewViewProvider(SidePanelProvider.viewType, provider));
}
