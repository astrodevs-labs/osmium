import './InteractPage.css';
import { useInteractPage } from './InteractPage.logic.ts';
import { VSCode } from '../../types';
import { FormProvider } from 'react-hook-form';
import { VSCodeButton, VSCodeDivider } from '@vscode/webview-ui-toolkit/react';
import { InteractContracts } from '../../components/InteractContracts/InteractContracts.tsx';
import { InteractParams } from '../../components/InteractParams/InteractParams.tsx';

export const InteractPage = (props: { vscode: VSCode }) => {
  const logic = useInteractPage(props.vscode);

  return (
    <div className="page-container">
      <FormProvider {...logic.form} >
        <form onSubmit={logic.form.handleSubmit(logic.onSubmit)}>
          <InteractContracts wallets={logic.wallets} contracts={logic.contracts} />
          <VSCodeDivider className="divider" />
          <InteractParams contracts={logic.contracts} />
          <VSCodeButton className="submit-button" type="submit">Send transaction</VSCodeButton>
        </form>
      </FormProvider>
    </div>
  );
};