import { VSCodeDropdown, VSCodeOption, VSCodeTextField } from '@vscode/webview-ui-toolkit/react';
import './DeployUsingContract.css';

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

export const DeployUsingContract = () => {

  return (
    <div>
      <div> DEPLOY USING CONTRACT </div>
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
      <label htmlFor="dropdown" className='label'>File:</label>
        <VSCodeDropdown id="dropdown">
          <VSCodeOption>My token - ERC20.sol</VSCodeOption>
        </VSCodeDropdown>
      </div>
      <div className="dropdown-container">
        <label htmlFor="dropdown" className='label'>Environment:</label>
        <VSCodeDropdown id="dropdown">
          <VSCodeOption>Remix VM</VSCodeOption>
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