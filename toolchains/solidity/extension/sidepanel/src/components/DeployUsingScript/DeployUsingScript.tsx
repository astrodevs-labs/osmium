import { VSCodeDropdown, VSCodeOption } from '@vscode/webview-ui-toolkit/react';
import './DeployUsingScript.css';

const script = [
  {
    address: '0x0999003...3333',
    balance: '0.12 ETH',
  },
  {
    address: '0x0999003...3333',
    balance: '0.12 ETH',
  },
  {
    address: '0x0999003...3333',
    balance: '0.12 ETH',
  },
];

const accounts = [
  {
    address: '0x0999003...3333',
    balance: '0.12 ETH',
  },
  {
    address: '0x0999003...3333',
    balance: '0.12 ETH',
  },
  {
    address: '0x0999003...3333',
    balance: '0.12 ETH',
  },
];

export const DeployUsingScript = () => {

  return (
    <div>
      <div> DEPLOY USING SCRIPT </div>
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
        <label htmlFor="dropdown" className='label'>Select script:</label>
        <VSCodeDropdown id="dropdown">
          {
            script.map((script) => (
              <VSCodeOption>{script.address} ({script.balance})</VSCodeOption>
            ))
          }
        </VSCodeDropdown>
      </div>
    </div>
  );
};