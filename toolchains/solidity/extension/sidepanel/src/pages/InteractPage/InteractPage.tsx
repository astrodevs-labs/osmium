import './InteractPage.css';
import { InteractParamsContainer } from '../../components/InteractParamsContainer/InteractParamsContainer.tsx';
import { InteractContractContainer } from '../../components/InteractContractContainer/InteractContractContainer.tsx';
import { VSCodeDivider } from '@vscode/webview-ui-toolkit/react/index';
import { InteractButton } from '../../components/InteractButton/InteractButton.tsx';

export const InteractPage = () => {
  return (<div className="page-container">
    <InteractContractContainer />
    <VSCodeDivider className='divider'/>
    <InteractParamsContainer />
    <VSCodeDivider className='divider'/>
    <InteractButton />
  </div>);
};