import './InteractParams.css';
import { Contract } from '../../../../src/actions/ContractRepository.ts';
import { useInteractParams } from './InteractParams.logic.ts';
import { VSCodeTextField } from '@vscode/webview-ui-toolkit/react';

export const InteractParams = (props: { contracts: Contract[] }) => {
  const logic = useInteractParams(props.contracts);

  const displayParams = logic.inputs && logic.inputs.length > 0;

  return (
    <>
      {displayParams &&
        <div className="params-container">
          {logic.inputs.map((input, index) => {
            return <VSCodeTextField className="text-field" {...logic.form.register(`inputs.${index}` as const, {
              required: true,
            })}>
              {input.type} {input.name}
            </VSCodeTextField>;
          })}
        </div>
      }
    </>
  );
};