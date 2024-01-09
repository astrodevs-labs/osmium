import { workspace, ExtensionContext } from 'vscode';
import {
	LanguageClient,
} from 'vscode-languageclient/node';
import { createLinterClient } from './linter';
import { createFoundryCompilerClient } from './foundry-compiler';
import { createSlitherClient } from './slither';
import registerForgeFmtLinter from "./fmt-wrapper";

let slitherClient: LanguageClient;
let linterClient: LanguageClient;
let foundryCompilerClient: LanguageClient;

export async function activate(context: ExtensionContext) {
	linterClient = createLinterClient(context);
	foundryCompilerClient = createFoundryCompilerClient(context);
	slitherClient = createSlitherClient(context);

	context.subscriptions.push(linterClient);
	context.subscriptions.push(foundryCompilerClient);
	context.subscriptions.push(slitherClient);

	registerForgeFmtLinter(context);

	
	const folders = workspace.workspaceFolders;
	if (folders) {
		const files = await workspace.findFiles('**/*.sol', `${folders[0].uri.fsPath}/**`);
		files.forEach(file => {
			if (!file.path.includes('forge-std')) {
				workspace.openTextDocument(file);
			}
		});
	}

}
