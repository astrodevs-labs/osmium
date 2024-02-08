import { Action, State } from '../contexts/InteractContext.tsx';

export const interactReducer = (state: State, action: Action) => {
  switch (action.type) {
    case 'SET_WALLETS': {
      return {
        ...state,
        wallets: action.wallets,
        selectedWallet: action.wallets.length ? action.wallets[0] : undefined,
      };
    }
    case 'SET_CONTRACTS': {
      const functions = !action.contracts.length ? undefined : action.contracts[0].abi.map((abi) => {
        if (abi.type === 'function') {
          return abi;
        }
      });

      let selectedFunction = undefined;

      if (functions && functions.length) {
        selectedFunction = functions[0]?.name;
      }

      return {
        ...state,
        contracts: action.contracts,
        selectedContract: action.contracts.length ? action.contracts[0] : undefined,
        selectedFunction: selectedFunction,
      };
    }
    case 'SET_SELECTED_WALLET': {
      return {
        ...state,
        selectedWallet: action.wallet,
      };
    }
    case 'SET_SELECTED_CONTRACT': {
      const functions = action.contract.abi.map((abi) => {
        if (abi.type === 'function') {
          return abi;
        }
      });

      return {
        ...state,
        selectedContract: action.contract,
        selectedFunction: functions.length ? functions[0]?.name : undefined,
      };
    }
    case 'SET_SELECTED_FUNCTION': {
      return {
        ...state,
        selectedFunction: action.func,
      };
    }
    case 'SET_GAS_LIMIT': {
      return {
        ...state,
        gasLimit: action.gasLimit,
      };
    }
    case 'SET_VALUE': {
      return {
        ...state,
        value: action.value,
      };
    }
    case 'SET_UNIT': {
      return {
        ...state,
        unit: action.unit,
      };
    }
    default: {
      throw Error('Unknown action: ' + action);
    }
  }
};