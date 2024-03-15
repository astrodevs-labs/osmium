import "./DeployPage.css";
import { DeployUsingScript } from '../../components/DeployUsingScript/DeployUsingScript.tsx';
import { DeployUsingContract } from '../../components/DeployUsingContract/DeployUsingContract.tsx';
import { useDeployPageScript, useDeployPageContract } from './DeployPage.logic.ts';
import { VSCode } from '../../types';
import { FormProvider } from "react-hook-form";

export const DeployPage = (props: { vscode: VSCode }) => {
  const logicScript = useDeployPageScript(props.vscode);
  const logicContract = useDeployPageContract(props.vscode);

  return (
  <div className="page-container">
    <FormProvider {...logicScript.form}>
      <form onSubmit={logicScript.form.handleSubmit(logicScript.onSubmit)}>
        <DeployUsingScript wallets={logicScript.wallets} scripts={logicScript.scripts} />
      </form>
    </FormProvider>
    <FormProvider {...logicContract.form}>
      <form onSubmit={logicContract.form.handleSubmit(logicContract.onSubmit)}>
        <DeployUsingContract wallets={logicContract.wallets} contracts={logicContract.contracts} />
      </form>
    </FormProvider>
  </div>);
};