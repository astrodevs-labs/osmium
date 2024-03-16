import { Contracts } from '../../../../src/actions/deploy.ts';
import { useFormContext } from 'react-hook-form';
import { DFormContract } from '../../types';
import { useEffect, useMemo } from 'react';

export const useDeployContractsParams = (contracts: Contracts[]) => {
  const form = useFormContext<DFormContract>();
  const selectedContractFile = form.watch('contract');
  const inputs = useMemo(() => {
    const res: any[] = [];

    if (selectedContractFile) {
      const selectedContract = contracts.find(contract => contract.path === selectedContractFile);
      if (selectedContract) {
        const constructorAbi = selectedContract.abi?.find(abi => abi.type === 'constructor');
        if (constructorAbi && constructorAbi.type === 'constructor') {
          res.push(...constructorAbi.inputs);
        }
      }
    }
    return res;
  }, [selectedContractFile, contracts]);

  useEffect(() => {
    return () => {
      inputs.forEach((_, idx) => {
        form.resetField(`inputs.${idx}`);
      })
    }
  }, [inputs]);
  
  return {
    inputs,
    form,
  };
};
