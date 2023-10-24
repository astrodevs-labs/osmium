/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */
import {
	createConnection,
	TextDocuments,
	Diagnostic,
	DiagnosticSeverity,
	ProposedFeatures,
	InitializeParams,
	DidChangeConfigurationNotification,
	CompletionItem,
	CompletionItemKind,
	TextDocumentPositionParams,
	TextDocumentSyncKind,
	InitializeResult
} from 'vscode-languageserver/node';
import {create_extension} from '../dist';

// Create a connection for the server, using Node's IPC as a transport.
// Also include all preview / proposed LSP features.
const connection = createConnection(ProposedFeatures.all);


	

const sendRequest = (method: string, params: any) => {
	return connection.sendRequest(method, params);
};

const sendNotification = (method: string, params: any) => {
	connection.sendNotification(method, params);
};

const extension = create_extension(sendRequest, sendNotification);

connection.onInitialize((params: InitializeParams) => {
	return extension.onRequest('initialize', params);
});

connection.onRequest((method: string, params: any) => {
	connection.console.log(`onRequest: ${method}`);
	
	return extension.onRequest(method, params);
});

connection.onNotification((method: string, params: any) => {
	extension.onNotification(method, params);
});

connection.listen();