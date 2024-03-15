import { useFormContext } from 'react-hook-form';
import { DFormContract } from '../../types';

export const useDeployContract = () => {
  const form = useFormContext<DFormContract>();
  const { formState: { errors } } = form;
  return { form, errors};
};