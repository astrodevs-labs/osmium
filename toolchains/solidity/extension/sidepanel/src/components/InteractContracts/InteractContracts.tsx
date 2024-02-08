import { VSCodeDropdown, VSCodeOption, VSCodeTextField } from '@vscode/webview-ui-toolkit/react';
import './InteractContracts.css';
import { useInteractContracts } from './InteractContracts.logic.ts';

export const InteractContracts = () => {
  const logic = useInteractContracts();

  return (
    <div>
      <div className="dropdown-container">
        <label htmlFor="dropdown-wallets" className="label">Select account:</label>
        <VSCodeDropdown id="dropdown-wallets">
          {
            logic.state.wallets.map((wallet) => (
              <VSCodeOption onClick={() => {
                logic.handleWalletChange(wallet);
              }}>{wallet.address} ({wallet.name})</VSCodeOption>
            ))
          }
        </VSCodeDropdown>
      </div>
      <div className="dropdown-container">
        <label htmlFor="dropdown-contracts" className="label">Select contract:</label>
        <VSCodeDropdown id="dropdown-contracts">
          {
            logic.state.contracts.map((contract) => (
              <VSCodeOption onClick={() => {
                logic.handleContractChange(contract);
              }}>{contract.address} ({contract.name})</VSCodeOption>
            ))
          }
        </VSCodeDropdown>
      </div>
      <div className="dropdown-container">
        <label htmlFor="dropdown" className="label">Select function:</label>
        <VSCodeDropdown id="dropdown">
          {
            logic.state.selectedContract?.abi.map((abi) => {
              if (abi.type === 'function') {
                return <VSCodeOption onClick={() => {
                  logic.handleFunctionChange(abi.name);
                }}>{abi.name}</VSCodeOption>;
              }
            })
          }
        </VSCodeDropdown>
      </div>
      <div className="gas-limit-container">
        <VSCodeTextField className="gas-limit-textfield" onChange={logic.handleGasLimitChange}>Gas
          limit</VSCodeTextField>
      </div>
      <div className="value-container">
        <label className="label">Value:</label>
        <div className="value-field-container">
          <VSCodeTextField className="value-textfield" onChange={logic.handleValueChange} />
          <VSCodeDropdown onChange={logic.handleUnitChange} className="value-dropdown" id="dropdown">
            <VSCodeOption>Wei</VSCodeOption>
            <VSCodeOption>Gwei</VSCodeOption>
            <VSCodeOption>Eth</VSCodeOption>
          </VSCodeDropdown>
        </div>
      </div>
    </div>
  );
};