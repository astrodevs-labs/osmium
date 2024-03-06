import { VSCodeDropdown, VSCodeOption, VSCodeTextField, VSCodeButton, VSCodeDivider } from '@vscode/webview-ui-toolkit/react';
import './DeployUsingContract.css';
import { Wallet } from "../../../../src/actions/WalletRepository.ts";
import { Contract } from "../../../../src/actions/deploy.ts";
import { useDeployContract } from "./DeployContract.logic.ts";
import { FormProvider, SubmitHandler } from "react-hook-form";
import { DFormContract } from "../../types";
export const DeployUsingContract = (
  { wallets, contracts }: { wallets: Wallet[]; contracts: Contract[] },
) => {
  const logic = useDeployContract();

  const onSubmit: SubmitHandler<DFormContract> = (data) => {
    console.log(data);
  };

  return (
    <FormProvider {...logic.form}>
      <form onSubmit={logic.form?.handleSubmit(onSubmit)}>
        <div>
          <div> DEPLOY USING CONTRACT </div>
          <div className="dropdown-container">
            <label htmlFor="dropdown-wallets" className="label">
              Select account:
            </label>
            <VSCodeDropdown
              id="dropdown-wallets"
              {...logic.form?.register("wallet", {
                required: true,
              })}
            >
              {wallets?.map((wallet) => (
                <VSCodeOption value={wallet.address}>
                  {wallet.name} - {wallet.address}
                </VSCodeOption>
              ))}
            </VSCodeDropdown>
          </div>
          <div className="dropdown-container">
          <label htmlFor="dropdown" className='label'>Select contract:</label>
          <VSCodeDropdown id="dropdown">
              {contracts?.map((contracts) => (
                <VSCodeOption>{contracts.path} ({contracts.name})</VSCodeOption>
              ))}
            </VSCodeDropdown>
          </div>
          <div className="dropdown-container">
            <label htmlFor="dropdown-environment" className='label'>Environment:</label>
              <div className="environment-container">
                <VSCodeDropdown id="dropdown-environment" className='dropdown-environment'>
                  <VSCodeOption>Remix VM</VSCodeOption>
                </VSCodeDropdown>
                <VSCodeButton className="add-wallet-button" onClick={() => {
                }}>Add</VSCodeButton>
              </div>
          </div>
          <div className="gas-limit-container">
            <VSCodeTextField className='gas-limit-textfield' value='300000' type="text">Gas limit</VSCodeTextField>
          </div>
          <div className="value-container">
            <label className='label'>Value:</label>
            <div className='value-field-container'>
              <VSCodeTextField className='value-textfield' value='0' type="text"/>
              <VSCodeDropdown className='value-dropdown' id="dropdown">
                <VSCodeOption>Wei</VSCodeOption>
                <VSCodeOption>Gwei</VSCodeOption>
                <VSCodeOption>ETH</VSCodeOption>
              </VSCodeDropdown>
            </div>
          </div>
        </div>
        <VSCodeDivider className='divider'/>
        <VSCodeButton className="submit-button" type="submit"> 
          Deploy with contract
        </VSCodeButton>
        </form>
    </FormProvider>
  );
};