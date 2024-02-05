import './InteractPage.css';
import { InteractParamsContainer } from '../../components/InteractParamsContainer/InteractParamsContainer.tsx';
import { InteractContractContainer } from '../../components/InteractContractContainer/InteractContractContainer.tsx';
import { VSCodeDivider } from '@vscode/webview-ui-toolkit/react/index';

export const InteractPage = () => {
  return (<div className="page-container">
    <InteractContractContainer />
    <VSCodeDivider className='divider'/>
    <InteractParamsContainer />
  </div>);
};