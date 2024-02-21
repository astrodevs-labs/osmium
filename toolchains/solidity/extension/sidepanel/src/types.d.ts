/* eslint-disable @typescript-eslint/no-explicit-any */
export type VSCode = any;

export interface IFormInput {
  wallet: string;
  contract: string;
  function: string;
  gasLimit: number;
  value: number;
  valueUnit: 'wei' | 'gwei' | 'ether';
  inputs: any[];
}