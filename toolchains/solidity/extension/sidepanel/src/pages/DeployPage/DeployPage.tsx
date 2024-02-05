import "./DeployPage.css";
import { DeployParamsContainer } from '../../components/DeployParamsContainer/DeployParamsContainer.tsx';
import { DeployUsingScript } from '../../components/DeployUsingScript/DeployUsingScript.tsx';
import { DeployUsingContract } from '../../components/DeployUsingContract/DeployUsingContract.tsx';
import { DeployButton } from '../../components/DeployButton/DeployButton.tsx';
import { VSCodeDivider } from '@vscode/webview-ui-toolkit/react/index';

export const DeployPage = () => {
  return (<div className="page-container">
    <DeployUsingScript />
    <VSCodeDivider className='divider'/>
    <DeployUsingContract />
    <VSCodeDivider className='divider'/>
    <DeployParamsContainer />
    <VSCodeDivider className='divider'/>
    <DeployButton />
  </div>);
};