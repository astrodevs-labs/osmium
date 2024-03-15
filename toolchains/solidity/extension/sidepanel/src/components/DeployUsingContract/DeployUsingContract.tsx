import { VSCodeDropdown, VSCodeOption, VSCodeTextField, VSCodeButton, VSCodeDivider } from '@vscode/webview-ui-toolkit/react';
import './DeployUsingContract.css';
import { Wallet } from "../../../../src/actions/WalletRepository.ts";
import { Contract } from "../../../../src/actions/deploy.ts";
import { useDeployContract } from "./DeployContract.logic.ts";
// import { DeployContractsParams } from "../DeployContractsParams/DeployContractsParams.tsx";
export const DeployUsingContract = (
  { wallets, contracts }: { wallets: Wallet[]; contracts: Contract[] },
) => {
  const logic = useDeployContract();

  return (
    <div>
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
        <VSCodeDropdown id="dropdown"
          {...logic.form?.register('contract', { required: true })}
        >
            {contracts?.map((contracts) => (
              <VSCodeOption>{contracts.path} ({contracts.name})</VSCodeOption>
            ))}
          </VSCodeDropdown>
        </div>
        <div className="dropdown-container">
          <label htmlFor="dropdown-environment" className='label'>Environment:</label>
            <div className="environment-container">
              <VSCodeDropdown id="dropdown-environment" className='dropdown-environment'
              {...logic.form?.register('environment', { required: true })}
              >
                <VSCodeOption>Remix VM</VSCodeOption>
              </VSCodeDropdown>
              <VSCodeButton className="add-wallet-button" onClick={() => {
              }}>Add</VSCodeButton>
            </div>
        </div>
        <div className="gas-limit-container">
          <VSCodeTextField className='gas-limit-textfield' {...logic.form?.register('gasLimit', {
            required: true,
            valueAsNumber: true,
          })}>Gas 
          limit</VSCodeTextField>
        {logic.errors.gasLimit && <span className="error-message">Invalid number</span>}
        </div>
        <div className="value-container">
          <label className='label'>Value:</label>
          <div className='value-field-container'>
            <VSCodeTextField className='value-textfield' {...logic.form?.register('value', {
              required: true,
              valueAsNumber: true,
            })}/>
            <VSCodeDropdown className='value-dropdown' id="dropdown" {...logic.form?.register('valueUnit', {
              required: true,
            })}>
              <VSCodeOption value="wei">Wei</VSCodeOption>
              <VSCodeOption value="gwei">Gwei</VSCodeOption>
              <VSCodeOption value="ether">Eth</VSCodeOption>
            </VSCodeDropdown>
          </div>
          {logic.errors.value && <span className="error-message">Invalid number</span>}
        </div>
      </div>
      <VSCodeDivider className='divider'/>
      {/* <DeployContractsParams contracts={contracts} /> */}
      <VSCodeButton className="submit-button" type="submit"> 
        Deploy with contract
      </VSCodeButton>
    </div>
  );
};