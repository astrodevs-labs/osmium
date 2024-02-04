import {useMessageHandler} from "./hooks/useMessageHandler.ts";
import {VSCodePanels, VSCodePanelTab, VSCodePanelView} from "@vscode/webview-ui-toolkit/react";
import './App.css';

function App() {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    //const vscode = acquireVsCodeApi();

    const messageHandlers = [
        {
            type: "answer",
            // eslint-disable-next-line @typescript-eslint/no-explicit-any
            callback: (data: any) => {
                console.log("answered", data);
            }
        },
        {
            type: "click2",
            callback: () => {
                console.log("clicked2");
            }
        }
    ]

    useMessageHandler(messageHandlers);

    return (
        <div className="app-container">
            <VSCodePanels className="app-container">
                <VSCodePanelTab id="tab-interact">INTERACT</VSCodePanelTab>
                <VSCodePanelTab id="tab-deploy">DEPLOY</VSCodePanelTab>
                <VSCodePanelView id="view-interact">
                    <p className="test">Interact</p>
                </VSCodePanelView>
                <VSCodePanelView id="view-deploy">
                    Deploy
                </VSCodePanelView>
            </VSCodePanels>

            {/*<VSCodeButton onClick={() => {
                vscode.postMessage({type: "post", value: "post"});
            }}>
                CLICK ME
            </VSCodeButton>*/}
        </div>
    )
}

export default App
