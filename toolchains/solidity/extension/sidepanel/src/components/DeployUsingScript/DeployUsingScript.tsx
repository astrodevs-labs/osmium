import {
  VSCodeButton,
  VSCodeDropdown,
  VSCodeOption,
  VSCodeDivider
} from "@vscode/webview-ui-toolkit/react";
import { Wallet } from "../../../../src/actions/WalletRepository.ts";
import "./DeployUsingScript.css";
import { Script } from "../../../../src/actions/deploy.ts";
import { useDeployScript } from "./DeployScript.logic.ts";

export const DeployUsingScript = (
  { wallets, scripts }: { wallets: Wallet[]; scripts: Script[] },
) => {
  const logic = useDeployScript();

  return (
    <div>
      <div>
        <div>DEPLOY USING SCRIPT</div>
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
          <label htmlFor="dropdown" className="label">Select script:</label>
          <VSCodeDropdown 
            id="dropdown"
            {...logic.form?.register("script", {
              required: true,
            })}
          >
            {scripts?.map((scripts) => (
              <VSCodeOption>{scripts.path} ({scripts.name})</VSCodeOption>
            ))}
          </VSCodeDropdown>
        </div>
      </div>
      <VSCodeDivider className='divider'/>
      <VSCodeButton className="submit-button" type="submit" >
        Deploy with script
      </VSCodeButton>
      <VSCodeDivider className='divider'/>
    </div>
  );
};
