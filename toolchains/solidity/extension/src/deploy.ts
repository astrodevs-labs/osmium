import { exec } from "child_process";
import { workspace } from "vscode";
import * as path from 'path';
import * as os from 'os';
import yaml from 'js-yaml';

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

async function getScriptFolder(): Promise<string> {
    const foundryConfigContent = await workspace.fs.readFile(workspace.workspaceFolders[0].uri.with({ path: path.join(workspace.workspaceFolders[0].uri.path, 'foundry.toml') }));
    const parsedFoundryConfig = yaml.load(foundryConfigContent.toString());

    return parsedFoundryConfig.script ?? 'script';
}

async function getContractFolder(): Promise<string> {
    const foundryConfigContent = await workspace.fs.readFile(workspace.workspaceFolders[0].uri.with({ path: path.join(workspace.workspaceFolders[0].uri.path, 'foundry.toml') }));
    const parsedFoundryConfig = yaml.load(foundryConfigContent.toString());

    return parsedFoundryConfig.contract ?? 'src';
}

async function getContracts(): Promise<Contract[]> {
    const contracts: Contract[] = [];
    const contractFolder = await getContractFolder();
    const contractFiles = await workspace.findFiles(`**/${contractFolder}/*.sol`);
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
    const scriptFolder = await getScriptFolder();
    const scriptFiles = await workspace.findFiles(`**/${scriptFolder}/*.sol`);
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
