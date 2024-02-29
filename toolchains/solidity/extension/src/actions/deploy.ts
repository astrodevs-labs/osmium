import { exec } from "child_process";
import * as path from 'path';
import { workspace } from "vscode";
import * as toml from "toml";

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
    try {
        const foundryConfigContent = await workspace.fs.readFile(workspace.workspaceFolders![0].uri.with({ path: path.join(workspace.workspaceFolders![0].uri.path, 'foundry.toml') }));
        const parsedFoundryConfig : any = toml.parse(foundryConfigContent.toString());
        return parsedFoundryConfig.script ?? 'script';
    } catch (error) {
        console.error("Error reading foundry.toml file:", error);
        return "not found";
    }
}

async function getContractFolder(): Promise<string> {
    try {
        const foundryConfigContent = await workspace.fs.readFile(workspace.workspaceFolders![0].uri.with({ path: path.join(workspace.workspaceFolders![0].uri.path, 'foundry.toml') }));
        const parsedFoundryConfig : any = toml.parse(foundryConfigContent.toString());
        return parsedFoundryConfig.contract ?? 'src';
    } catch (error) {
        console.error("Error reading foundry.toml file:", error);
        return "not found";
    }
}

async function getOutFolder(): Promise<string> {
    try {
        const foundryConfigContent = await workspace.fs.readFile(workspace.workspaceFolders![0].uri.with({ path: path.join(workspace.workspaceFolders![0].uri.path, 'foundry.toml') }));
        const parsedFoundryConfig : any = toml.parse(foundryConfigContent.toString());
        return parsedFoundryConfig.out ?? 'out';
    } catch (error) {
        console.error("Error reading foundry.toml file:", error);
        return "not found";
    }
}

async function getAbiFile(contractName: string, outFolder: string): Promise<string> {
    try {
        const abiFilePath = path.join(outFolder, contractName, `${contractName}.json`);
        const abiFileBuffer = await workspace.fs.readFile(workspace.workspaceFolders![0].uri.with({ path: abiFilePath }));
        const abiFileContent = Buffer.from(abiFileBuffer).toString('utf-8');
        return abiFileContent;
    } catch (error) {
        console.error("Error reading abi file:", error);
        return "not found";
    }
}

export async function getContracts(): Promise<Contract[]> {
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

export async function getScripts(): Promise<Script[]> {
    const scripts: Script[] = [];
    const scriptFolder = await getScriptFolder();
    const scriptFiles = await workspace.findFiles(`**/${scriptFolder}/*.s.sol`);
    const outFolder = await getOutFolder();
    console.log("outFolder: ", outFolder);
    for (const scriptFile of scriptFiles) {
        const scriptContentBuffer = await workspace.fs.readFile(scriptFile);
        const scriptContent = Buffer.from(scriptContentBuffer).toString('utf-8');
        const contractNameRegex = /contract\s+(\w+)\s+is\s+Script/g;
        let scriptNameMatch = contractNameRegex.exec(scriptContent);
        while (scriptNameMatch !== null) {
            const scriptName = scriptNameMatch[1];
            const abi = await getAbiFile(scriptName, outFolder);
            console.log("abi: ", abi);
            const script = {
                name: path.basename(scriptFile.path, '.s.sol'),
                path: scriptFile.path,
                abi: JSON.parse(abi).abi,
            };
            console.log("final infos of script: ", script);
            scripts.push(script);
        }

    }
    console.log("before return: ", scripts);
    return scripts;
}

export async function deployContract(network: number, contract: Contract, verify: boolean, cstrArgs: string): Promise<void> {
    const verifyStr = verify ? '--verify' : '';
    exec(`forge create ${contract.path}:${contract.name} -c ${network} ${verifyStr} --contructor-args ${cstrArgs}`, (error, _stdout, _stderr) => {
        if (error) {
            throw error;
        }
    });
}

export async function deployScript(network: number, script: Script, verify: boolean): Promise<void> {
    const verifyStr = verify ? '--verify' : '';
    exec(`forge script ${script.path}:${script.name} -c ${network} ${verifyStr}`, (error, _stdout, _stderr) => {
        if (error) {
            throw error;
        }
    });
}

export async function verifyContract(network: number, contract: Contract): Promise<void> {
    // TODO load the contructor args path from out
    exec(`forge verify-contract ${contract.path} ${contract.address} -c ${network}`, (error, _stdout, _stderr) => {
        if (error) {
            throw error;
        }
    });
}