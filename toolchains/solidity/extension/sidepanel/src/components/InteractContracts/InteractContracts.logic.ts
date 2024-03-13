import { useFormContext } from 'react-hook-form';
import { Contract } from '../../../../src/actions/ContractRepository.ts';
import { IFormInput } from '../../types';

export const useInteractContracts = (contracts: Contract[]) => {
  const { register, watch, formState: { errors } } = useFormContext<IFormInput>();
  const selectedContract = watch('contract');

  const functions = contracts?.find((contract) => contract.address === selectedContract)?.abi?.map((abi) => {
    if (abi.type === 'function') {
      return abi.name;
    }
  }) || [];

  return { register, selectedContract, functions, errors };
};