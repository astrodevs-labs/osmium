import { SubmitHandler, useForm } from 'react-hook-form';
import { DFormScript, VSCode, DFormContract } from '../../types';
import { useEffect, useState } from 'react';
import { Wallet } from '../../../../src/actions/WalletRepository.ts';
import { Script } from '../../../../src/actions/deploy.ts';
import { Contracts } from '../../../../src/actions/deploy.ts';

export enum MessageTypeScript {
  GET_WALLETS = 'GET_WALLETS',
  WALLETS = 'WALLETS',
  GET_SCRIPTS = 'GET_SCRIPTS',
  SCRIPTS = 'SCRIPTS',
}

export enum MessageTypeContract {
  GET_WALLETS = 'GET_WALLETS',
  WALLETS = 'WALLETS',
  GET_DEPLOY_CONTRACTS = 'GET_DEPLOY_CONTRACTS',
  DEPLOY_CONTRACTS = 'DEPLOY_CONTRACTS',
  EDIT_ENVIRONMENT = 'EDIT_ENVIRONMENT',
}

export const useDeployPageScript = (vscode: VSCode) => {
  const [wallets, setWallets] = useState<Wallet[]>([]);
  const [scripts, setScripts] = useState<Script[]>([]);
  const form = useForm<DFormScript>({
    defaultValues: {
      wallet: '',
      script: '',
    },
  });

  const onSubmit: SubmitHandler<DFormScript> = (data) => {
    console.log(data);
  };

  useEffect(() => {
    if (!vscode) {
      return;
    }
    vscode.postMessage({ type: MessageTypeScript.GET_WALLETS });
    vscode.postMessage({ type: MessageTypeScript.GET_SCRIPTS });
  }, [vscode]);

  useEffect(() => {
    const listener = (event: WindowEventMap['message']) => {
      switch (event.data.type) {
        case MessageTypeScript.WALLETS: {
          form.setValue('wallet', event.data.wallets && event.data.wallets.length ? event.data.wallets[0].address : '');
          setWallets(event.data.wallets);
          break;
        }
        case MessageTypeScript.SCRIPTS: {
          form.setValue('script', event.data.scripts && event.data.scripts.length ? event.data.scripts[0].name : '');
          setScripts(event.data.scripts);
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
    scripts,
    onSubmit,
  };
};

export const useDeployPageContract = (vscode: VSCode) => {
  const [wallets, setWallets] = useState<Wallet[]>([]);
  const [contracts, setContracts] = useState<Contracts[]>([]);
  const form = useForm<DFormContract>({
    defaultValues: {
      wallet: '',
      contract: '',
      environment: 'Remix VM',
      value: 0,
      valueUnit: 'wei',
      gasLimit: 300000,
    },
  });

  const onSubmit: SubmitHandler<DFormContract> = (data) => {
    console.log(data);
  };

  useEffect(() => {
    if (!vscode) {
      return;
    }
    vscode.postMessage({ type: MessageTypeContract.GET_WALLETS });
    vscode.postMessage({ type: MessageTypeContract.GET_DEPLOY_CONTRACTS });
  }, [vscode]);

  useEffect(() => {
    const listener = (event: WindowEventMap['message']) => {
      switch (event.data.type) {
        case MessageTypeContract.WALLETS: {
          form.setValue('wallet', event.data.wallets && event.data.wallets.length ? event.data.wallets[0].address : '');
          setWallets(event.data.wallets);
          break;
        }
        case MessageTypeContract.DEPLOY_CONTRACTS: {
          form.setValue('contract', event.data.contracts && event.data.contracts.length ? event.data.contracts[0].path : '');
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