import *  as path from 'path';
import * as fs from 'fs';
import { Chain, Abi } from 'viem';

export interface Contract {
    name: string;
    address: `0x${string}`;
    abi: Abi;
    chain: Chain;
    rpc: `ws://${string}` | `wss://${string}` | `http://${string}` | `https://${string}`;
    usedWallet: `0x${string}`;
}

export type Contracts = Contract[];

export class ContractRepository
{
    private _contracts: Contracts = [];
    private _contractsPath: string;

    constructor(workspacePath: string) {
        this._contractsPath = path.join(workspacePath, '.osmium', 'contracts.json');
        this._load();
    }

    private _load(): void {
        if (!fs.existsSync(this._contractsPath)) {
            this._contracts = [];
            fs.writeFileSync(this._contractsPath, JSON.stringify({ contracts: this._contracts }));
        } else {
            const raw = fs.readFileSync(this._contractsPath);
            const json = JSON.parse(raw.toString());
            this._contracts = json.contracts;
        }
    }

    private _save(): void {
        const json = JSON.stringify({ contracts: this._contracts });
        fs.writeFileSync(this._contractsPath, json, { encoding: 'utf-8' });
    }

    getContracts(): Contracts {
        return this._contracts;
    }

    getContract(name: Contract['address']): Contract | undefined {
        return this._contracts.find(c => c.address === name);
    }

    createContract(contract: Contract): void {
        this._contracts.push(contract);
        this._save();
    }
}