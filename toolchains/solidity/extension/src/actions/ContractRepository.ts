import * as path from "path";
import * as fs from "fs";
import { Abi } from "viem";

export interface Contract {
  name: string;
  address: `0x${string}`;
  abi: Abi;
  chainId: number;
  rpc:
    | `ws://${string}`
    | `wss://${string}`
    | `http://${string}`
    | `https://${string}`;
  usedWallet: `0x${string}`;
}

export type Contracts = Contract[];

export class ContractRepository {
  private _contracts: Contracts = [];
  private _contractsPath: string;

  constructor(workspacePath: string) {
    this._contractsPath = path.join(workspacePath, ".osmium", "contracts.json");
    this.load();
  }

  private _save(): void {
    const json = JSON.stringify({ contracts: this._contracts });
    fs.writeFileSync(this._contractsPath, json, { encoding: "utf-8" });
  }

  public load(): void {
    if (!fs.existsSync(this._contractsPath)) {
      this._contracts = [];
      fs.writeFileSync(
        this._contractsPath,
        JSON.stringify({ contracts: this._contracts }),
      );
    } else {
      const raw = fs.readFileSync(this._contractsPath);
      const json = JSON.parse(raw.toString());
      this._contracts = json.contracts;
    }
  }

  public getContracts(): Contracts {
    return this._contracts;
  }

  public getContract(name: Contract["address"]): Contract | undefined {
    return this._contracts.find((c) => c.address === name);
  }

  public createContract(contract: Contract): void {
    this._contracts.push(contract);
    this._save();
  }
}
