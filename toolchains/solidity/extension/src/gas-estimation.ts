import { exec } from "child_process";
import * as vscode from "vscode";

type GasReport = {
  average: bigint;
  min?: bigint;
  max?: bigint;
  median?: bigint;
};

type Function = {
  name: string,
  line: number
};

type Report = Map<string, Map<string, GasReport>>;

function isForgeInstalled(): boolean {
  try {
    exec("forge -version", (error: any, _stdout: any, _stderr: any) => {
      if (error) {
        throw error;
      }
    });
    return true;
  } catch (error) {
    return false;
  }
}

// Contracts needs to be formatted like this : ["contract1.sol:Contract1", "contract2.sol:Contract2"]
function getGasReport(contracts: string[]): Report {
  const report: Report = new Map();

  // Gas estimation from the tests
  exec("forge test --gas-report", (error: any, _stdout: any, _stderr: any) => {
    if (error) {
      throw error;
    }

    let contractName = "";
    _stdout.split("\n").forEach((line: string) => {
      const lineParts = line.split("|");
      if (lineParts.length === 6) {
        const trimmedLineParts = lineParts.map((part) => part.trim());
        if (trimmedLineParts[0] !== "" && trimmedLineParts[1] === "" && trimmedLineParts[2] === "" && trimmedLineParts[3] === "" && trimmedLineParts[4] === "" && trimmedLineParts[5] === "") {
          contractName = trimmedLineParts[0];
          return;
        }

        if (trimmedLineParts[0] !== "" && trimmedLineParts[1] !== "" && trimmedLineParts[2] !== "" && trimmedLineParts[3] !== "" && trimmedLineParts[4] !== "" && trimmedLineParts[5] !== ""
          && lineParts[0].replace("-", "") !== "" && lineParts[1].replace("-", "") !== "" && lineParts[2].replace("-", "") !== "" && lineParts[3].replace("-", "") !== "" && lineParts[4].replace("-", "") !== "" && lineParts[5].replace("-", "") !== ""
          && trimmedLineParts[0] !== "Function Name") {
          const functionName = trimmedLineParts[1];
          const min = BigInt(trimmedLineParts[2]);
          const average = BigInt(trimmedLineParts[3]);
          const median = BigInt(trimmedLineParts[4]);
          const max = BigInt(trimmedLineParts[5]);

          report.set(contractName, new Map());
          report.get(contractName)?.set(functionName, { min, average, median, max });
        }
      }
    });
  });

  // Gas estimation from the contracts inspection
  contracts.forEach((contract) => {
    // TODO find config path
    exec(`forge inspect ${contract} gasEstimates`, (error: any, _stdout: any, _stderr: any) => {
      if (error) {
        throw error;
      }

      const json = JSON.parse(_stdout);
      const internalFunctions = Object.keys(json.internal);
      const externalFunctions = Object.keys(json.external);

      internalFunctions.forEach((functionName) => {
        const res: string = json.internal[functionName];
        if (res !== "infinite") {
          if (report.has(contract)) {
            report.get(contract)?.set(functionName, { average: BigInt(res) });
          } else {
            report.set(contract, new Map());
            report.get(contract)?.set(functionName, { average: BigInt(res) });
          }
        }
      });
      externalFunctions.forEach((functionName) => {
        const res: string = json.external[functionName];
        if (res !== "infinite") {
          if (report.has(contract)) {
            report.get(contract)?.set(functionName, { average: BigInt(res) });
          } else {
            report.set(contract, new Map());
            report.get(contract)?.set(functionName, { average: BigInt(res) });
          }
        }
      });
    });
  });
  return report;
}

function getXthWord(line: string, index: number): string {
  const splittedLine = line.split(" ");

  let count = 0;
  for (const word of splittedLine) {
    if (word !== "") {
      if (index === count) {
        return word;
      }
      count += 1;
    }
  }
  return "";
}

function getContractsInsideFile(content: string, path: string): string[] {
  const contracts: string[] = [];
  const lines = content.split("\n");

  lines.forEach((line) => {
    if (getXthWord(line, 0) === "contract") {
      const contractName = getXthWord(line, 1);
      contracts.push(`${path}:${contractName}`);
    }
  });
  return contracts;
}

function getFunctionsInsideContract(content: string, contractName: string): Function[] {
  const functions: Function[] = [];
  const lines = content.split("\n");

  let start = false;
  let bracketsCount = 0;
  let currentContractName = "";
  lines.forEach((line, index) => {
    const firstWord = getXthWord(line, 0);
    const secondWord = getXthWord(line, 1);
    if (firstWord === "contract") {
      currentContractName = secondWord;
      if (contractName === currentContractName) {
        start = true;
      }
    }
    if (start) {
      bracketsCount += (line.split("{").length - 1) - (line.split("}").length - 1);
      if (bracketsCount === -1) {
        return functions;
      }
      if (firstWord === "function") {
        const functionName = secondWord.split("(")[0];
        functions.push({
          name: functionName,
          line: index + 1
        });
      }
    }
  });

  return functions;
}

function gasReport(content: string, path: string) {
  if (!isForgeInstalled()) {
    return;
  }
  const contracts = getContractsInsideFile(content, path);
  const report = getGasReport(contracts);
  const functionsPerContract: Map<string, Function[]> = new Map();
  contracts.map((contract) => {
    const functions = getFunctionsInsideContract(content, contract.split(":")[1]);
    functionsPerContract.set(contract.split(":")[0], functions);
  });

  for (const [contract, functions] of functionsPerContract) {
    for (const func of functions) {
      // TODO: display the report close to the functions definitions
      const gas = report.get(contract)?.get(func.name)?.average;
    }
  }
}

export function registerGasEstimation() {
  vscode.workspace.onDidOpenTextDocument((document) => {
    // gas estimate only the main contracts
    if (!document.fileName.includes("lib") && !document.fileName.includes("test"))
      gasReport(document.getText(), document.uri.path);
  });

  vscode.workspace.onDidSaveTextDocument((document) => {
    gasReport(document.getText(), document.uri.path);
  });
}
