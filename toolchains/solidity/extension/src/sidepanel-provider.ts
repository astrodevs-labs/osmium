import * as vscode from "vscode";
import { ContractRepository } from "./actions/ContractRepository";
import { WalletRepository } from "./actions/WalletRepository";
import { Script, getScripts } from "./actions/deploy";
import { Contract, getContracts } from "./actions/deploy";
import { Interact } from "./actions/Interact";
import {window} from "vscode";
import {Address} from "viem";

enum MessageType {
  GET_WALLETS = "GET_WALLETS",
  WALLETS = "WALLETS",
  GET_INTERACT_CONTRACTS = "GET_INTERACT_CONTRACTS",
  INTERACT_CONTRACTS = "INTERACT_CONTRACTS",
  GET_DEPLOY_CONTRACTS = "GET_DEPLOY_CONTRACTS",
  DEPLOY_CONTRACTS = "DEPLOY_CONTRACTS",
  WRITE = "WRITE",
  WRITE_RESPONSE = "WRITE_RESPONSE",
  READ = "READ",
  GET_SCRIPTS = "GET_SCRIPTS",
  SCRIPTS = "SCRIPTS",
  READ_RESPONSE = "READ_RESPONSE",
  EDIT_WALLETS = 'EDIT_WALLETS',
  EDIT_CONTRACTS = 'EDIT_CONTRACTS',
}

type Message = {
  type: MessageType;
  data: any;
};

enum InputAction {
    ADD = "Add",
    REMOVE = "Remove",
}

function getNonce(): string {
  let text: string = "";
  const possible: string =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  for (let i = 0; i < 32; i++) {
    text += possible.charAt(Math.floor(Math.random() * possible.length));
  }
  return text;
}

export class SidePanelProvider implements vscode.WebviewViewProvider {
  public static readonly viewType = "osmium.sidepanel";

  private _view?: vscode.WebviewView;

  private _contractRepository?: ContractRepository;
  private _walletRepository?: WalletRepository;
  private _interact?: Interact;
  private _scripts?: Script[];
  private _contracts?: Contract[];

  private _watcher?: vscode.FileSystemWatcher;

  constructor(private readonly _extensionUri: vscode.Uri) {}

  public async resolveWebviewView(
    webviewView: vscode.WebviewView,
    _context: vscode.WebviewViewResolveContext,
    _token: vscode.CancellationToken,
  ) {
    this._view = webviewView;

    if (vscode.workspace.workspaceFolders?.length) {
      this._contractRepository = new ContractRepository(
        vscode.workspace.workspaceFolders?.[0].uri.fsPath || "",
      );
      this._walletRepository = new WalletRepository(
        vscode.workspace.workspaceFolders?.[0].uri.fsPath || "",
      );

      this._interact = new Interact(
        this._contractRepository,
        this._walletRepository,
      );

      this._scripts = await getScripts();
      this._contracts = await getContracts();
      const pattern = new vscode.RelativePattern(
        vscode.workspace.workspaceFolders?.[0].uri.fsPath,
        ".osmium/*.json",
      );
      this._watcher = vscode.workspace.createFileSystemWatcher(pattern);

      this._watcher.onDidChange(async (uri) => {
        if (this._view) {
          if (uri.fsPath.endsWith("contracts.json")) {
            this._contractRepository?.load();
            await this._view.webview.postMessage({
              type: MessageType.INTERACT_CONTRACTS,
              contracts: this._contractRepository?.getContracts(),
            });
          } else {
            this._walletRepository?.load();
            await this._view.webview.postMessage({
              type: MessageType.WALLETS,
              wallets: this._walletRepository?.getWallets(),
            });
          }
        }
      });
    }

    webviewView.webview.options = {
      enableScripts: true,
      localResourceRoots: [this._extensionUri],
    };

    webviewView.webview.html = this._getHtmlForWebview(webviewView.webview);

    webviewView.webview.onDidReceiveMessage(async (message: Message) => {
      if (
        !this._view ||
        !this._contractRepository ||
        !this._walletRepository ||
        !this._interact ||
        !this._scripts ||
        !this._contracts
      ) {
        return;
      }
      switch (message.type) {
        case MessageType.GET_WALLETS:
          await this._view.webview.postMessage({
            type: MessageType.WALLETS,
            wallets: this._walletRepository.getWallets(),
          });
          break;
        case MessageType.GET_INTERACT_CONTRACTS:
          await this._view.webview.postMessage({
            type: MessageType.INTERACT_CONTRACTS,
            contracts: this._contractRepository.getContracts(),
          });
          break;
        case MessageType.GET_SCRIPTS:
          await this._view.webview.postMessage({
            type: MessageType.SCRIPTS,
            scripts: this._scripts,
          });
          break;
          case MessageType.GET_DEPLOY_CONTRACTS:
            await this._view.webview.postMessage({
              type: MessageType.DEPLOY_CONTRACTS,
              contracts: this._contracts,
            });
            break;
        case MessageType.WRITE:
          const writeResponse = await this._interact.writeContract({
            account: message.data.wallet,
            address: message.data.contract,
            abi: this._contractRepository.getContract(message.data.contract)!
              .abi,
            functionName: message.data.function,
            params: message.data.inputs,
          });
          await this._view.webview.postMessage({
            type: MessageType.WRITE_RESPONSE,
            response: writeResponse,
          });
          break;
        case MessageType.READ:
          const readResponse = await this._interact.readContract({
            contract: message.data.contract,
            method: message.data.function,
            params: message.data.inputs,
          });
          await this._view.webview.postMessage({
            type: MessageType.READ_RESPONSE,
            response: readResponse,
          });
          break;
        case MessageType.EDIT_WALLETS:
          const walletAction = await window.showQuickPick([InputAction.ADD, InputAction.REMOVE], {title: "Edit Wallets"});

          if (walletAction === InputAction.ADD) {
            const walletName = await window.showInputBox({prompt: "Enter wallet name"});
            const walletAddress = await window.showInputBox({prompt: "Enter wallet address"});
            const walletPk = await window.showInputBox({prompt: "Enter wallet private key"});
            const walletRpc = await window.showInputBox({prompt: "Enter wallet rpc"});

            if (!walletName || !walletAddress || !walletPk || !walletRpc) return;
            if (!walletAddress.startsWith("0x") || !walletPk.startsWith("0x")) return;
            if (!walletRpc.startsWith("http") && !walletRpc.startsWith("ws")) return;

            await this._walletRepository.createWallet({
              name: walletName,
              address: <Address>walletAddress,
              privateKey: <Address>walletPk,
              rpc: walletRpc,
            });
          }

          if (walletAction === InputAction.REMOVE) {
            const walletAddress = await window.showInputBox({prompt: "Enter wallet address"});
            if (!walletAddress) return;
            await this._walletRepository.deleteWallet(<Address>walletAddress);
          }

          break;
        case MessageType.EDIT_CONTRACTS:
          const contractAction = await window.showQuickPick([InputAction.ADD, InputAction.REMOVE], {title: "Edit Wallets"});

          if (contractAction === InputAction.ADD) {
            const contractName = await window.showInputBox({prompt: "Enter name"});
            const contractAddress = await window.showInputBox({prompt: "Enter address"});
            const contractAbi = await window.showInputBox({prompt: "Enter abi"});
            const contractRpc = await window.showInputBox({prompt: "Enter rpc"});
            const contractChainId = await window.showInputBox({prompt: "Enter chain id"});

            if (!contractName || !contractAddress || !contractAbi || !contractRpc || !contractChainId) return;
            if (!contractAddress.startsWith("0x")) return;
            if (!contractRpc.startsWith("http") && !contractRpc.startsWith("ws")) return;

            this._contractRepository.createContract({
              address: <Address>contractAddress,
              abi: JSON.parse(contractAbi),
              chainId: parseInt(contractChainId),
              name: contractName,
              rpc: <any>contractRpc,
            });
          }

          if (contractAction === InputAction.REMOVE) {
            const contractAddress = await window.showInputBox({prompt: "Enter address"});
            if (!contractAddress) return;
            await this._contractRepository.deleteContract(<Address>contractAddress);
          }

          break;
      }
    });
  }

  private _getHtmlForWebview(webview: vscode.Webview) {
    // Get the local path to main script run in the webview, then convert it to a uri we can use in the webview.
    const scriptUri = webview.asWebviewUri(
      vscode.Uri.joinPath(this._extensionUri, "sidepanel", "dist", "index.js"),
    );

    const styleUri = webview.asWebviewUri(
      vscode.Uri.joinPath(this._extensionUri, "sidepanel", "dist", "index.css"),
    );

    // Use a nonce to only allow a specific script to be run.
    const nonce = getNonce();

    return `<!doctype html>
        <html lang="en">
          <head>
            <meta charset="UTF-8" />
            <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${webview.cspSource}; script-src 'nonce-${nonce}';">
			<meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Panel</title>
            <script type="module" nonce="${nonce}" crossorigin src="${scriptUri}"></script>
            <link rel="stylesheet" crossorigin href="${styleUri}">
          </head>
          <body>
            <div id="root"></div>
          </body>
        </html>`;
  }
}
