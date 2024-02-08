/* eslint-disable @typescript-eslint/no-explicit-any */
import { useContext } from 'react';
import { InteractContext } from '../../contexts/InteractContext.tsx';
import { Wallet } from '../../../../src/actions/WalletRepository.ts';
import { Contract } from '../../../../src/actions/ContractRepository.ts';

export const useInteractContracts = () => {
  const { state, dispatch } = useContext(InteractContext);

  const handleWalletChange = (wallet: Wallet) => {
    dispatch({ type: 'SET_SELECTED_WALLET', wallet: wallet });
  };

  const handleContractChange = (contract: Contract) => {
    dispatch({ type: 'SET_SELECTED_CONTRACT', contract: contract });
  };

  const handleFunctionChange = (func: string) => {
    dispatch({ type: 'SET_SELECTED_FUNCTION', func: func });
  };

  const handleGasLimitChange = (event: any) => {
    const gasLimit = event.target.value;

    dispatch({ type: 'SET_GAS_LIMIT', gasLimit: parseInt(gasLimit) });
  };

  const handleValueChange = (event: any) => {
    const value = event.target.value;
    dispatch({ type: 'SET_VALUE', value: BigInt(value) });
  };

  const handleUnitChange = (event: any) => {
    dispatch({ type: 'SET_UNIT', unit: event.target.value });
  };

  return {
    state,
    handleWalletChange,
    handleContractChange,
    handleFunctionChange,
    handleGasLimitChange,
    handleValueChange,
    handleUnitChange,
  };
};