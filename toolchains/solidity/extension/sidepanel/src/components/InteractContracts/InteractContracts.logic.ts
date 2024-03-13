import { useFormContext } from 'react-hook-form';
import { Contract } from '../../../../src/actions/ContractRepository.ts';
import {IFormInput, VSCode} from '../../types';
import {MessageType} from "../../pages/InteractPage/InteractPage.logic.ts";

export const useInteractContracts = (contracts: Contract[], vscode: VSCode) => {
  const { register, watch, formState: { errors } } = useFormContext<IFormInput>();
  const selectedContract = watch('contract');

  const functions = contracts?.find((contract) => contract.address === selectedContract)?.abi?.map((abi) => {
    if (abi.type === 'function') {
      return abi.name;
    }
  }) || [];

  const editWallet = () => {
    vscode.postMessage({ type: MessageType.EDIT_WALLETS });
  }

  const editContract = () => {
    vscode.postMessage({ type: MessageType.EDIT_CONTRACTS });
  }

  return { register, selectedContract, functions, errors, editWallet, editContract };
};