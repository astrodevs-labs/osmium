import "./DeployPage.css";
import { DeployUsingScript } from '../../components/DeployUsingScript/DeployUsingScript.tsx';
import { DeployUsingContract } from '../../components/DeployUsingContract/DeployUsingContract.tsx';
import { useDeployPageScript, useDeployPageContract } from './DeployPage.logic.ts';
import { VSCode } from '../../types';

export const DeployPage = (props: { vscode: VSCode }) => {
  const logicScript = useDeployPageScript(props.vscode);
  const logicContract = useDeployPageContract(props.vscode);

  return (
  <div className="page-container">
    <DeployUsingScript wallets={logicScript.wallets} scripts={logicScript.scripts} />
    <DeployUsingContract wallets={logicContract.wallets} contracts={logicContract.contracts} />
  </div>);
};