import { VSCodePanels, VSCodePanelTab, VSCodePanelView } from '@vscode/webview-ui-toolkit/react';
import { DeployPage } from './pages/DeployPage/DeployPage.tsx';
import { InteractPage } from './pages/InteractPage/InteractPage.tsx';
import { useEffect, useState } from 'react';
import './App.css';

function App() {
  //const messageHandlers = [
  //  {
  //    type: 'answer',
  //    // eslint-disable-next-line @typescript-eslint/no-explicit-any
  //    callback: (data: any) => {
  //      console.log('answered', data);
  //    },
  //  },
  //  {
  //    type: 'click2',
  //    callback: () => {
  //      console.log('clicked2');
  //    },
  //  },
  //];
//
  //useMessageHandler(messageHandlers);

  const [vscode, setVscode] = useState();

  useEffect(() => {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const tmp = acquireVsCodeApi();
    setVscode(tmp);
  }, []);

  return (
    <>
      <VSCodePanels>
        <VSCodePanelTab id="tab-interact">INTERACT</VSCodePanelTab>
        <VSCodePanelTab id="tab-deploy">DEPLOY</VSCodePanelTab>
        <VSCodePanelView id="view-interact">
          <InteractPage vscode={vscode} />
        </VSCodePanelView>
        <VSCodePanelView id="view-deploy">
          <DeployPage />
        </VSCodePanelView>
      </VSCodePanels>

      {/*<VSCodeButton onClick={() => {
                vscode.postMessage({type: "post", value: "post"});
            }}>
                CLICK ME
            </VSCodeButton>*/}
    </>
  );
}

export default App;
