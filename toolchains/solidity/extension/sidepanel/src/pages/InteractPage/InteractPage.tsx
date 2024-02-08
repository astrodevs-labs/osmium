/* eslint-disable @typescript-eslint/no-explicit-any */
import './InteractPage.css';
import { InteractParamsContainer } from '../../components/InteractParams/InteractParams.tsx';
import { InteractContracts } from '../../components/InteractContracts/InteractContracts.tsx';
import { VSCodeButton, VSCodeDivider } from '@vscode/webview-ui-toolkit/react';
import { InteractProvider } from '../../contexts/InteractContext.tsx';

export const InteractPage = (props: { vscode: any }) => {
  return (
    <InteractProvider vscode={props.vscode}>
      <div className="page-container">
        <InteractContracts />
        <VSCodeDivider className="divider" />
        <InteractParamsContainer />
        <VSCodeDivider className="divider" />
        <VSCodeButton>Send transaction</VSCodeButton>
      </div>
    </InteractProvider>
  );
};