import "./DeployPage.css";
import { DeployUsingScript } from '../../components/DeployUsingScript/DeployUsingScript.tsx';
import { DeployUsingContract } from '../../components/DeployUsingContract/DeployUsingContract.tsx';
import { VSCodeDivider,  VSCodeButton} from '@vscode/webview-ui-toolkit/react/index';
import { useDeployPageScript } from './DeployPage.logic.ts';
import { VSCode } from '../../types';

export const DeployPage = (props: { vscode: VSCode }) => {
  const logicScript = useDeployPageScript(props.vscode);

  return (<div className="page-container">
    <DeployUsingScript wallets={logicScript.wallets} scripts={logicScript.scripts} />
    <DeployUsingContract />
    <VSCodeDivider className='divider'/>
    <VSCodeDivider className='divider'/>
    <VSCodeButton>Deploy with contract</VSCodeButton>
  </div>);
};