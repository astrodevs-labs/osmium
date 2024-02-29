import { SubmitHandler, useForm } from 'react-hook-form';
import { DFormScript, VSCode } from '../../types';
import { useEffect, useState } from 'react';
import { Wallet } from '../../../../src/actions/WalletRepository.ts';
import { Script } from '../../../../src/actions/deploy.ts';

enum MessageType {
  GET_WALLETS = 'GET_WALLETS',
  WALLETS = 'WALLETS',
  GET_SCRIPTS = 'GET_SCRIPTS',
  SCRIPTS = 'SCRIPTS',
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
    vscode.postMessage({ type: MessageType.GET_WALLETS });
    vscode.postMessage({ type: MessageType.GET_SCRIPTS });
  }, [vscode]);

  useEffect(() => {
    const listener = (event: WindowEventMap['message']) => {
      switch (event.data.type) {
        case MessageType.WALLETS: {
          form.setValue('wallet', event.data.wallets && event.data.wallets.length ? event.data.wallets[0].address : '');
          setWallets(event.data.wallets);
          break;
        }
        case MessageType.SCRIPTS: {
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