import './DeployContracts.css';
import { Contracts } from '../../../../src/actions/deploy.ts';
import { useDeployContractsParams } from './DeployContractsParams.logic.ts';
import { VSCodeTextField } from '@vscode/webview-ui-toolkit/react';

export const DeployContractsParams = (props: { contracts: Contracts[] }) => {
  const logic = useDeployContractsParams(props.contracts);

  const displayParams = logic.inputs && logic.inputs.length > 0;
  return (
    <>
      {displayParams &&
        <div className="params-container">
          {logic.inputs?.map((input, index) => {
            return <>
              <VSCodeTextField className="text-field" {...logic.form.register(`inputs.${index}` as const, {
                required: true,
                valueAsNumber: input.type.includes('int'),
              })}>
                {input.name} ({input.type})
              </VSCodeTextField>
            </>;
          })}
        </div>
      }
    </>
  );
};