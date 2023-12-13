import { LanguageClient } from 'vscode-languageclient/node';
import * as vscode from 'vscode';
import { testAll, testContract, testFunction, FileResult, hasForge } from './foundry-test';

enum ItemType {
    File,
    ContractCase,
    TestCase
}

export class TestManager {
    public testController: vscode.TestController;
    private fileChangedEmitter : vscode.EventEmitter<vscode.Uri>;
    private watchingTests : Map<vscode.TestItem | 'ALL', vscode.TestRunProfile | undefined>;
    private testData = new WeakMap<vscode.TestItem, ItemType>();
    

    constructor(private client: LanguageClient, private workspace: string) {        
        this.testController = vscode.tests.createTestController("solidityTestController", "Solidity test controller");
        this.fileChangedEmitter = new vscode.EventEmitter<vscode.Uri>();
        this.watchingTests = new Map<vscode.TestItem | 'ALL', vscode.TestRunProfile | undefined>();

        this.testController.resolveHandler = (test) =>  {
          console.log("controller resolve");
          return this.resolve(test);
        };
        this.testController.createRunProfile("Run tests", vscode.TestRunProfileKind.Run, (request, token) => this.runHandler(false, request, token))
        this.testController.createRunProfile("Debug tests", vscode.TestRunProfileKind.Run, (request, token) => this.runHandler(true, request, token))

        vscode.workspace.onDidOpenTextDocument(this.parseTestsInDocument);

        console.log("Test manager created"); 
    }

    private async runHandler(
        shouldDebug: boolean,
        request: vscode.TestRunRequest,
        token: vscode.CancellationToken
      ) {
        console.log("Run handler called");
        const run = this.testController.createTestRun(request);
        const queue: vscode.TestItem[] = [];
        // const getType = (testItem: vscode.TestItem) => this.testData.get(testItem);
      
        // Loop through all included tests, or all known tests, and add them to our queue
        if (request.include) {
          request.include.forEach(test => queue.push(test));
        } else {
            this.testController.items.forEach(test => queue.push(test));
        }
      
        // For every test that was queued, try to run it. Call run.passed() or run.failed().
        // The `TestMessage` can contain extra information, like a failing location or
        // a diff output. But here we'll just give it a textual message.
        while (queue.length > 0 && !token.isCancellationRequested) {
          const test = queue.pop()!;
      
          // Skip tests the user asked to exclude
          if (request.exclude?.includes(test)) {
            continue;
          }
      
          const date = Date.now();
          try {
            switch (this.testData.get(test)!) {
              case ItemType.File:
                // If we're running a file and don't know what it contains yet, parse it now
                if (test.children.size === 0) {
                  await this.parseTestsInFileContents(test);
                }
              break;
              case ItemType.ContractCase:
                  //get result form foundry wrapper for contract test
                  const contractResult = await testContract(this.workspace, test.label);
                  const contractTime = Date.now() - date;
                  if (this.analyzeTestResults(contractResult)) {
                      run.passed(test, contractTime);
                  } else {
                      run.failed(test, new vscode.TestMessage("Contract test failed"), contractTime);
                  }
              break;
              case ItemType.TestCase:
                  //get result form foundry wrapper for test case
                  const functionResult = await testFunction(this.workspace, test.parent!.label, test.label);
                  const functionTime = Date.now() - date;
                  if (this.analyzeTestResults(functionResult)) {
                      run.passed(test, functionTime);
                  } else {
                      run.failed(test, new vscode.TestMessage("Contract test failed"), functionTime);
                  }
              break;
            }
          } catch (e: any) {
            run.appendOutput(JSON.stringify(e));
            run.failed(test, new vscode.TestMessage("Test failed"));
            if (e === "No forge found") {
                vscode.window.showErrorMessage("No forge found. Please install forge and make sure it's in your PATH");
            }
          }
      
          test.children.forEach(test => queue.push(test));
        }
      
        // Make sure to end the run after all tests have been executed:
        run.end();
      }

    private analyzeTestResults(result : FileResult) {
        console.log("analyzeTestResults");
        console.log(result);
        Object.values(result).forEach((suiteResult) => {
            Object.values(suiteResult.test_results).forEach((testResult) => {
                if (testResult.status !== "Success") {
                    return false;
                }
            });
        });
        return true;

    }
        

    
    private async getTestsPositions(content: string): Promise<any> {
        console.log("getTestsPositions");
        return this.client.sendRequest('osmium/getTestsPositions', {
            file_content: content
        });
    }

    private getOrCreateTestFileItem(uri: vscode.Uri) {
        console.log("getOrCreateTestFileItem");
        const existing = this.testController.items.get(uri.toString());
        if (existing) {
            return existing;
        }

        const file = this.testController.createTestItem(uri.toString(), uri.path.split('/').pop()!, uri);
        this.testData.set(file, ItemType.File);
        file.canResolveChildren = true;
        this.testController.items.add(file);
        return file;
    }

    private async resolve(test?: vscode.TestItem) {
        console.log("resolve");
        if (!test) {
            await this.discoverAllFilesInWorkspace();
          } else {
            await this.parseTestsInFileContents(test);
          }
    }

    private async discoverAllFilesInWorkspace() {
        console.log("discoverAllFilesInWorkspace");
        if (!vscode.workspace.workspaceFolders) {
          return []; // handle the case of no open folders
        }
      
        return Promise.all(
          vscode.workspace.workspaceFolders.map(async workspaceFolder => {
            const pattern = new vscode.RelativePattern(workspaceFolder, '**/*.t.sol');
            const watcher = vscode.workspace.createFileSystemWatcher(pattern);
      
            // When files are created, make sure there's a corresponding "file" node in the tree
            watcher.onDidCreate(uri => this.getOrCreateTestFileItem(uri));
            // When files change, re-parse them. Note that you could optimize this so
            // that you only re-parse children that have been resolved in the past.
            watcher.onDidChange(uri => this.parseTestsInFileContents(this.getOrCreateTestFileItem(uri)));
            // And, finally, delete TestItems for removed files. This is simple, since
            // we use the URI as the TestItem's ID.
            watcher.onDidDelete(uri => this.testController.items.delete(uri.toString()));
      
            for (const file of await vscode.workspace.findFiles(pattern)) {
             this.getOrCreateTestFileItem(file);
            }
      
            return watcher;
          })
        );
    }

    private parseTestsInDocument(e: vscode.TextDocument) {
        console.log("parseTestsInDocument");
        if (e.uri.scheme === 'file' && e.uri.path.endsWith('.t.sol')) {
          this.parseTestsInFileContents(this.getOrCreateTestFileItem(e.uri), e.getText());
        }
    }

    private async parseTestsInFileContents(file: vscode.TestItem, contents?: string) {
        console.log("parseTestsInFileContents");
        // If a document is open, VS Code already knows its contents. If this is being
        // called from the resolveHandler when a document isn't open, we'll need to
        // read them from disk ourselves.
        if (contents === undefined) {
          const rawContent = await vscode.workspace.fs.readFile(file.uri!);
          contents = new TextDecoder().decode(rawContent);
        }
      
        // some custom logic to fill in test.children from the contents...
        
        if (contents != undefined) {
            // CALL getTestPositions and fill children
            await this.getTestsPositions(contents).then((testPositions) => {
                testPositions.contracts.forEach((contract: any) => {
                    const contractName = contract.name.replace(" ", "");
                    const contractItem = this.testController.createTestItem(contractName, contract.name, file.uri);
                    contractItem.range = convertRange(contract.range);
                    console.log("Contract range", JSON.stringify(contractItem.range));
                    this.testData.set(contractItem, ItemType.ContractCase);
                    file.children.add(contractItem);

                    contract.tests.forEach((test: any) => {
                        const functionItem = this.testController.createTestItem(`${contractName}_${test.name}`, test.name, file.uri);
                        functionItem.range = convertRange(test.range);
                        console.log("Test range", JSON.stringify(functionItem.range));
                        this.testData.set(functionItem, ItemType.TestCase);
                        contractItem.children.add(functionItem)
                    });
                });
                
        });
        }
    }
}

function convertRange(lspRange: any): vscode.Range {
  const range = new vscode.Range(
    new vscode.Position(lspRange.start.line - 1, lspRange.start.character),
    new vscode.Position(lspRange.end.line - 1, lspRange.end.character),
  )
  console.log(range);
  return range;
}