import { exec } from "child_process";
import * as vscode from "vscode";

type GasReport = {
  average: bigint;
  min?: bigint;
  max?: bigint;
  median?: bigint;
}

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

function getContractsInsideFile(content: string, path: string): string[] {
  const contracts: string[] = [];
  const lines = content.split("\n");

  lines.forEach((line) => {
    if (line.includes("contract")) {
      const contractName = line.split(" ")[1];
      contracts.push(`${path}:${contractName}`);
    }
  });
  return contracts;
}

export function gasReport(content: string, path: string) {
  if (!isForgeInstalled()) {
    return;
  }
  const contracts = getContractsInsideFile(content, path);
  const report = getGasReport(contracts);

  // TODO: display the report close to the functions definitions
}
