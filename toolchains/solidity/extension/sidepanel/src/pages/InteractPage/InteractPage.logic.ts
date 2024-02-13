import { SubmitHandler, useForm } from 'react-hook-form';
import { IFormInput, VSCode } from '../../types';
import { useEffect, useState } from 'react';
import { Wallet } from '../../../../src/actions/WalletRepository.ts';
import { Contract } from '../../../../src/actions/ContractRepository.ts';

enum MessageType {
  GET_WALLETS = 'GET_WALLETS',
  WALLETS = 'WALLETS',
  GET_CONTRACTS = 'GET_CONTRACTS',
  CONTRACTS = 'CONTRACTS',
}

export const useInteractPage = (vscode: VSCode) => {
  const [wallets, setWallets] = useState<Wallet[]>([]);
  const [contracts, setContracts] = useState<Contract[]>([]);
  const form = useForm<IFormInput>({
    defaultValues: {
      wallet: '',
      contract: '',
      function: '',
      value: 0,
      valueUnit: 'wei',
      gasLimit: 300000,
    },
  });

  const onSubmit: SubmitHandler<IFormInput> = (data) => {
    console.log(data);
  };

  useEffect(() => {
    if (!vscode) {
      return;
    }
    vscode.postMessage({ type: MessageType.GET_WALLETS });
    vscode.postMessage({ type: MessageType.GET_CONTRACTS });
  }, [vscode]);

  useEffect(() => {
    const listener = (event: WindowEventMap['message']) => {
      switch (event.data.type) {
        case MessageType.WALLETS: {
          form.setValue('wallet', event.data.wallets ? event.data.wallets[0].address : '');
          setWallets(event.data.wallets);
          break;
        }
        case MessageType.CONTRACTS: {
          form.setValue('contract', event.data.contracts ? event.data.contracts[0].address : '');
          setContracts(event.data.contracts);
          break;
        }
        default: {
          throw Error('Unknown command: ' + event.type);
        }
      }
    };
    window.addEventListener('message', listener);
    return () => window.removeEventListener('message', listener);
  }, []);

  return {
    form,
    vscode,
    wallets,
    contracts,
    onSubmit,
  };
};