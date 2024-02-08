/* eslint-disable @typescript-eslint/no-explicit-any */
import { createContext, Dispatch, ReactNode, useEffect, useReducer } from 'react';
import { Contract } from '../../../src/actions/ContractRepository.ts';
import { Wallet } from '../../../src/actions/WalletRepository.ts';
import { interactReducer } from '../reducers/interact-reducer.ts';

export type Action =
  | { type: 'SET_WALLETS', wallets: Wallet[] }
  | { type: 'SET_CONTRACTS', contracts: Contract[] }
  | { type: 'SET_SELECTED_WALLET', wallet: Wallet }
  | { type: 'SET_SELECTED_CONTRACT', contract: Contract }
  | { type: 'SET_SELECTED_FUNCTION', func: string }
  | { type: 'SET_GAS_LIMIT', gasLimit: number }
  | { type: 'SET_VALUE', value: bigint }
  | { type: 'SET_UNIT', unit: string };

export type State = {
  wallets: Wallet[];
  contracts: Contract[];
  selectedWallet: Wallet | undefined;
  selectedContract: Contract | undefined;
  selectedFunction: string | undefined;
  gasLimit: number;
  value: bigint;
  unit: string;
}

enum MessageType {
  GET_WALLETS = 'GET_WALLETS',
  WALLETS = 'WALLETS',
  GET_CONTRACTS = 'GET_CONTRACTS',
  CONTRACTS = 'CONTRACTS',
}

export const InteractContext = createContext<{
  state: State;
  dispatch: Dispatch<Action>;
}>({
  state: {
    wallets: [],
    contracts: [],
    selectedWallet: undefined,
    selectedContract: undefined,
    selectedFunction: undefined,
    gasLimit: 300000,
    value: BigInt(0),
    unit: 'wei',
  },
  dispatch: () => null,
});

export const InteractProvider = (props: { children: ReactNode, vscode?: any }) => {
  const initialState: State = {
    wallets: [],
    contracts: [],
    selectedContract: undefined,
    selectedWallet: undefined,
    selectedFunction: undefined,
    gasLimit: 300000,
    value: BigInt(0),
    unit: 'wei',
  };

  const [state, dispatch] = useReducer(
    interactReducer,
    initialState,
  );

  useEffect(() => {
    if (!props.vscode) {
      return;
    }
    props.vscode.postMessage({ type: MessageType.GET_WALLETS });
    props.vscode.postMessage({ type: MessageType.GET_CONTRACTS });
  }, [props.vscode]);

  // TO REMOVE DEBUG
  useEffect(() => {
    console.log('state', state);
  }, [state]);

  useEffect(() => {
    const listener = (event: any) => {
      switch (event.data.type) {
        case MessageType.WALLETS: {
          dispatch({ type: 'SET_WALLETS', wallets: event.data.wallets });
          break;
        }
        case MessageType.CONTRACTS: {
          dispatch({ type: 'SET_CONTRACTS', contracts: event.data.contracts });
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

  return (
    <InteractContext.Provider value={{ state, dispatch }}>
      {props.children}
    </InteractContext.Provider>
  );
};

