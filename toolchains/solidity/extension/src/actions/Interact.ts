/* eslint-disable @typescript-eslint/no-explicit-any */
import {
  Abi,
  createPublicClient,
  createWalletClient,
  getContract,
  http,
  webSocket,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { ContractRepository } from "./ContractRepository";
import { WalletRepository } from "./WalletRepository";
import * as chains from "viem/chains";

interface ReadContractOptions {
  contract: `0x${string}`;
  method: string;
  params?: any[];
}

interface WriteContractOptions {
  account: `0x${string}`;
  address: `0x${string}`;
  abi: Abi;
  functionName: string;
  params?: any[];
}

const getChain = (id: number) => {
  for (const chain of Object.values(chains)) {
    if ("id" in chain) {
      if (chain.id === id) {
        return chain;
      }
    }
  }
};

export class Interact {
  private contractRepository: ContractRepository;
  private walletRepository: WalletRepository;

  constructor(
    contractRepository: ContractRepository,
    walletRepository: WalletRepository,
  ) {
    this.contractRepository = contractRepository;
    this.walletRepository = walletRepository;
  }

  public async readContract({
    contract,
    method,
    params,
  }: ReadContractOptions): Promise<any> {
    const contractInfos = this.contractRepository.getContract(contract);
    if (!contractInfos) {
      throw new Error(`contract ${contract} not found`);
    }

    const viemContract = getContract({
      address: contractInfos.address,
      abi: contractInfos.abi,
      client: createPublicClient({
        chain: getChain(contractInfos.chainId),
        transport: contractInfos.rpc.startsWith("ws")
          ? webSocket(contractInfos.rpc)
          : http(contractInfos.rpc),
      }),
    });

    return await viemContract.read[method](<any>params);
  }

  public async writeContract({
    account,
    address,
    abi,
    functionName,
    params,
  }: WriteContractOptions): Promise<any> {
    const walletInfos = this.walletRepository.getWallet(account);
    if (!walletInfos) {
      throw new Error(`wallet ${account} not found`);
    }
    const contract = this.contractRepository.getContract(address);
    if (!contract) {
      throw new Error(`contract ${address} not found`);
    }

    const walletClient = createWalletClient({
      chain: getChain(contract.chainId),
      transport: walletInfos.rpc.startsWith("ws")
        ? webSocket(walletInfos.rpc)
        : http(walletInfos.rpc),
      account: privateKeyToAccount(walletInfos.privateKey),
    });

    const viemContract = getContract({
      address,
      abi,
      client: walletClient,
    });

    return await viemContract.write[functionName](<any>params);
  }
}
