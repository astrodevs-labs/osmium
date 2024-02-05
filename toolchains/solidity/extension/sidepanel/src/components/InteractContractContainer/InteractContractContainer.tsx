import { VSCodeDropdown, VSCodeOption, VSCodeTextField } from '@vscode/webview-ui-toolkit/react';
import './InteractContractContainer.css';

const contracts = [
  {
    address: '0x0999003...3333',
    name: 'SecureSafe',
  },
  {
    address: '0x0999003...3333',
    name: 'SecureSafe',
  },
  {
    address: '0x0999003...3333',
    name: 'SecureSafe',
  },
];

const accounts = [
  {
    address: '0x0999003...3333',
    balance: '0.1ETH',
  },
  {
    address: '0x0999003...3333',
    balance: '0.1ETH',
  },
  {
    address: '0x0999003...3333',
    balance: '0.1ETH',
  },
];

export const InteractContractContainer = () => {

  return (
    <div>
      <div className="dropdown-container">
        <label htmlFor="dropdown" className='label'>Select account:</label>
        <VSCodeDropdown id="dropdown">
          {
            accounts.map((account) => (
              <VSCodeOption>{account.address} ({account.balance})</VSCodeOption>
            ))
          }
        </VSCodeDropdown>
      </div>
      <div className="dropdown-container">
        <label htmlFor="dropdown" className='label'>Select contract:</label>
        <VSCodeDropdown id="dropdown">
          {
            contracts.map((contract) => (
              <VSCodeOption>{contract.address} ({contract.name})</VSCodeOption>
            ))
          }
        </VSCodeDropdown>
      </div>
      <div className="dropdown-container">
        <label htmlFor="dropdown" className='label'>Select function:</label>
        <VSCodeDropdown id="dropdown">
          <VSCodeOption>Add</VSCodeOption>
          <VSCodeOption>Remove</VSCodeOption>
          <VSCodeOption>Clear</VSCodeOption>
        </VSCodeDropdown>
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
  );
};