import { exec } from "child_process";
import { workspace } from "vscode";
import * as path from 'path';

export type Contract = {
    name: string;
    path: string;
    abi: any[];
    address?: string;
}

export type Script = {
    path: string;
    name: string;
}

async function getContracts(): Promise<Contract[]> {
    const contracts: Contract[] = [];
    // TODO read from config file the contract path
    const contractFiles = await workspace.findFiles('**/src/*.sol');
    for (const contractFile of contractFiles) {
        const contractContent = await workspace.fs.readFile(contractFile);
        // TODO find a way to get the contract name
        // TODO handle multiple contracts inside a single file
        // TODO get the abi from the out directory based on the contract name
        const contract = {
            name: path.basename(contractFile.path, '.sol'),
            path: contractFile.path,
            abi: JSON.parse(contractContent.toString()).abi,
        };
        contracts.push(contract);
    }
    return contracts;
}

async function getScripts(): Promise<Script[]> {
    const scripts: Script[] = [];
    // TODO read from config file the script path
    const scriptFiles = await workspace.findFiles('**/script/*.sol');
    for (const scriptFile of scriptFiles) {
        // TODO find a way to get the script name
        // TODO handle multiple scripts inside a single file
        // TODO get the abi from the out directory based on the contract name
        const script = {
            name: path.basename(scriptFile.path, '.sol'),
            path: scriptFile.path,
        };
        scripts.push(script);
    }
    return [];
}

async function deployContract(network: number, contract: Contract, verify: boolean, cstrArgs: string): Promise<void> {
    const verifyStr = verify ? '--verify' : '';
    exec(`forge create ${contract.path}:${contract.name} -c ${network} ${verifyStr} --contructor-args ${cstrArgs}`, (error, _stdout, _stderr) => {
        if (error) {
            throw error;
        }
    });
}

async function deployScript(network: number, script: Script, verify: boolean): Promise<void> {
    const verifyStr = verify ? '--verify' : '';
    exec(`forge script ${script.path}:${script.name} -c ${network} ${verifyStr}`, (error, _stdout, _stderr) => {
        if (error) {
            throw error;
        }
    });
}

async function verifyContract(network: number, contract: Contract): Promise<void> {
    // TODO load the contructor args path from out
    exec(`forge verify-contract ${contract.path} ${contract.address} -c ${network}`, (error, _stdout, _stderr) => {
        if (error) {
            throw error;
        }
    });
}
