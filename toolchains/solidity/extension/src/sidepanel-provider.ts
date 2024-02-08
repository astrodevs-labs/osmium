import * as vscode from "vscode";
import { ContractRepository } from "./actions/ContractRepository";
import { WalletRepository } from "./actions/WalletRepository";

enum MessageType {
  GET_WALLETS = "GET_WALLETS",
  WALLETS = "WALLETS",
  GET_CONTRACTS = "GET_CONTRACTS",
  CONTRACTS = "CONTRACTS",
}

type Message = {
  type: MessageType;
  data: any;
};

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

  constructor(private readonly _extensionUri: vscode.Uri) {}

  public resolveWebviewView(
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
    }

    webviewView.webview.options = {
      // Allow scripts in the webview
      enableScripts: true,

      localResourceRoots: [this._extensionUri],
    };

    webviewView.webview.html = this._getHtmlForWebview(webviewView.webview);

    webviewView.webview.onDidReceiveMessage(async (message: Message) => {
      if (!this._view || !this._contractRepository || !this._walletRepository) {
        return;
      }
      switch (message.type) {
        case MessageType.GET_WALLETS:
          await this._view.webview.postMessage({
            type: MessageType.WALLETS,
            wallets: this._walletRepository.getWallets(),
          });
          break;
        case MessageType.GET_CONTRACTS:
          await this._view.webview.postMessage({
            type: MessageType.CONTRACTS,
            contracts: this._contractRepository.getContracts(),
          });
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
