import "./DeployPage.css";
import { DeployUsingScript } from '../../components/DeployUsingScript/DeployUsingScript.tsx';
import { DeployUsingContract } from '../../components/DeployUsingContract/DeployUsingContract.tsx';
import { useDeployPageScript, useDeployPageContract } from './DeployPage.logic.ts';
import { VSCode } from '../../types';
import { FormProvider } from "react-hook-form";
import { useInteractPage } from '../InteractPage/InteractPage.logic.ts';

export const DeployPage = (props: { vscode: VSCode }) => {
  const logicScript = useDeployPageScript(props.vscode);
  const logicContract = useDeployPageContract(props.vscode);
  const edit = useInteractPage(props.vscode);

  return (
  <div className="page-container">
    <FormProvider {...logicScript.form}>
      <form onSubmit={logicScript.form.handleSubmit(logicScript.onSubmit)}>
        <DeployUsingScript wallets={logicScript.wallets} scripts={logicScript.scripts} vscode={props.vscode} contracts={edit.contracts} />
      </form>
    </FormProvider>
    <FormProvider {...logicContract.form}>
      <form onSubmit={logicContract.form.handleSubmit(logicContract.onSubmit)}>
        <DeployUsingContract wallets={logicContract.wallets} deployContracts={logicContract.contracts} vscode={props.vscode} editContracts={edit.contracts}/>
      </form>
    </FormProvider>
  </div>);
};