import * as path from "path";
import * as fs from "fs";

export interface Wallet {
  name: string;
  address: `0x${string}`;
  privateKey: `0x${string}`;
  rpc: string;
}

export class WalletRepository {
  private _wallets: Wallet[] = [];
  private _walletsPath: string;

  constructor(workspacePath: string) {
    this._walletsPath = path.join(workspacePath, ".osmium", "wallets.json");
    this.load();
  }

  public load(): void {
    if (fs.existsSync(this._walletsPath)) {
      const walletData = fs.readFileSync(this._walletsPath, "utf8");
      const walletJson = JSON.parse(walletData);
      this._wallets = walletJson.wallets;
    } else {
      fs.writeFileSync(this._walletsPath, JSON.stringify({ wallets: [] }));
      this._wallets = [];
    }
  }

  public getWallets(): Wallet[] {
    return this._wallets;
  }

  public getWallet(address: `0x${string}`): Wallet | undefined {
    return this._wallets.find((w) => w.address === address);
  }

  public async createWallet(wallet: Wallet): Promise<Wallet> {
    if (this._wallets.find((w) => w.address === wallet.address)) {
      // replace
      this._wallets = this._wallets.map((w) => {
        if (w.address === wallet.address) {
          return wallet;
        }
        return w;
      });
    } else {
      this._wallets.push(wallet);
    }

    const walletJson = JSON.stringify({ wallets: this._wallets });
    fs.writeFileSync(this._walletsPath, walletJson, "utf8");
    return wallet;
  }
}
