import { useFormContext } from 'react-hook-form';
import { DFormContract } from '../../types';

export const useDeployContract = () => {
  const form = useFormContext<DFormContract>();

  return { form };
};