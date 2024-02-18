import './InteractContracts.css';
import { Wallet } from '../../../../src/actions/WalletRepository.ts';
import { Contract } from '../../../../src/actions/ContractRepository.ts';
import { useInteractContracts } from './InteractContracts.logic.ts';
import { VSCodeDropdown, VSCodeOption, VSCodeTextField } from '@vscode/webview-ui-toolkit/react';

export const InteractContracts = (props: { wallets: Wallet[], contracts: Contract[] }) => {
  const { wallets, contracts } = props;
  const logic = useInteractContracts(contracts);

  return (
    <div>
      <div className="dropdown-container">
        <label htmlFor="dropdown-wallets" className="label">Select account:</label>
        <VSCodeDropdown id="dropdown-wallets" {...logic.register('wallet', {
          required: true,
        })}>
          {wallets.map((wallet) => (
            <VSCodeOption value={wallet.address}>{wallet.name} - {wallet.address}</VSCodeOption>
          ))}
        </VSCodeDropdown>
      </div>
      <div className="dropdown-container">
        <label htmlFor="dropdown-contracts" className="label">Select contract:</label>
        <VSCodeDropdown id="dropdown-contracts" {...logic.register('contract', { required: true })}>
          {contracts.map((contract) => (
            <VSCodeOption value={contract.address}>{contract.name} - {contract.address}</VSCodeOption>
          ))}
        </VSCodeDropdown>
      </div>
      <div className="dropdown-container">
        <label htmlFor="dropdown-functions" className="label">Select function:</label>
        <VSCodeDropdown id="dropdown-functions" {...logic.register('function', { required: true })}>
          {logic.functions.map((func) => {
            if (!func) return null;
            return <VSCodeOption value={func}>{func}</VSCodeOption>;
          })}
        </VSCodeDropdown>
      </div>
      <div className="gas-limit-container">
        <VSCodeTextField className="gas-limit-textfield" {...logic.register('gasLimit', {
          required: true,
          valueAsNumber: true,
        })}>Gas
          limit</VSCodeTextField>
        {logic.errors.gasLimit && <span className="error-message">Invalid number</span>}
      </div>
      <div className="value-container">
        <label className="label">Value:</label>
        <div className="value-field-container">
          <VSCodeTextField className="value-textfield" {...logic.register('value', {
            required: true,
            valueAsNumber: true,
          })} />
          <VSCodeDropdown className="value-dropdown" id="dropdown" {...logic.register('valueUnit', {
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
  );
};