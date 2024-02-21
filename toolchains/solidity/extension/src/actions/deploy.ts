import { exec } from "child_process";
import * as yaml from 'js-yaml';
import * as path from 'path';
import { workspace } from "vscode";

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
    const foundryConfigContent = await workspace.fs.readFile(workspace.workspaceFolders![0].uri.with({ path: path.join(workspace.workspaceFolders![0].uri.path, 'foundry.toml') }));
    const parsedFoundryConfig : any = yaml.load(foundryConfigContent.toString());

    return parsedFoundryConfig.script ?? 'script';
}

async function getContractFolder(): Promise<string> {
    const foundryConfigContent = await workspace.fs.readFile(workspace.workspaceFolders![0].uri.with({ path: path.join(workspace.workspaceFolders![0].uri.path, 'foundry.toml') }));
    const parsedFoundryConfig : any = yaml.load(foundryConfigContent.toString());

    return parsedFoundryConfig.contract ?? 'src';
}

async function getOutFolder(): Promise<string> {
    const foundryConfigContent = await workspace.fs.readFile(workspace.workspaceFolders![0].uri.with({ path: path.join(workspace.workspaceFolders![0].uri.path, 'foundry.toml') }));
    const parsedFoundryConfig : any = yaml.load(foundryConfigContent.toString());

    return parsedFoundryConfig.out ?? 'out';
}

async function getAbiFile(contractName: string, outFolder: string): Promise<string> {
    const abiFilePath = path.join(outFolder, contractName, `${contractName}.json`);
    const abiFileBuffer = await workspace.fs.readFile(workspace.workspaceFolders![0].uri.with({ path: abiFilePath }));
    const abiFileContent = Buffer.from(abiFileBuffer).toString('utf-8');
    
    return abiFileContent;
}

async function getContracts(): Promise<Contract[]> {
    const contracts: Contract[] = [];
    const contractFolder = await getContractFolder();
    const contractFiles = await workspace.findFiles(`**/${contractFolder}/*.sol`);
    const outFolder = await getOutFolder();
    for (const contractFile of contractFiles) {
        const contractContentBuffer = await workspace.fs.readFile(contractFile);
        const contractContent = Buffer.from(contractContentBuffer).toString('utf-8');
        const contractNameRegex = /contract\s+(\w+)\s*\{/g;
        let contractNameMatch;
        while ((contractNameMatch = contractNameRegex.exec(contractContent)) !== null) {
            const contractName = contractNameMatch[1];
            const abi = await getAbiFile(contractName, outFolder);
            const contract = {
                name: contractName,
                path: contractFile.path,
                abi: JSON.parse(abi).abi,
            };
            contracts.push(contract);
        }
    }
    return contracts;
}

async function getScripts(): Promise<Script[]> {
    const scripts: Script[] = [];
    const scriptFolder = await getScriptFolder();
    const scriptFiles = await workspace.findFiles(`**/${scriptFolder}/*.sol`);
    const outFolder = await getOutFolder();
    for (const scriptFile of scriptFiles) {
        const scriptContentBuffer = await workspace.fs.readFile(scriptFile);
        const scriptContent = Buffer.from(scriptContentBuffer).toString('utf-8');
        const contractNameRegex = /contract\s+(\w+)\s*\{/g;
        let scriptNameMatch;
        while ((scriptNameMatch = contractNameRegex.exec(scriptContent)) !== null) {
            const scriptName = scriptNameMatch[1];
            const abi = await getAbiFile(scriptName, outFolder);
            const script = {
                name: path.basename(scriptFile.path, '.sol'),
                path: scriptFile.path,
                abi: JSON.parse(abi).abi,
            };
            scripts.push(script);
        }

    }
    return scripts;
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