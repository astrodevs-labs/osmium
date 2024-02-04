import {useEffect} from "react";

interface MessageHandler {
    type: string;
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    callback: (payload: any) => void;
}

export const useMessageHandler = (messagesHandlers: MessageHandler[]) => {
    useEffect(() => {
        const handler = (event: MessageEvent) => {
            const eventData = event.data;
            messagesHandlers.forEach(messageHandler => {
                if (messageHandler.type === eventData.type) {
                    messageHandler.callback(eventData.data);
                }
            });
        };
        window.addEventListener("message", handler);
        return () => {
            window.removeEventListener("message", handler);
        };
    }, [messagesHandlers]);
}