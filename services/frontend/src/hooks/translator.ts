'use client'

import { useCallback, useEffect, useState } from 'react';
import { TranslateRequest } from '@/model/translate_request';

const useTranslator = (backend_url: string): [boolean, string, (text: string) => void] => {
    const [isConnected, setIsConnected] = useState<boolean>(false);
    const [socket, setSocket] = useState<WebSocket>();

    const [translated, setTranslated] = useState('');

    const requestTranslation = useCallback((text: string) => {
        const request: TranslateRequest = {
            command: {
                type: 'translate',
                payload: text,
            }
        };

        let incoming = '';
        setTranslated(incoming);

        if (socket) {
            socket.onmessage = (ev) => {
                incoming = incoming + ' ' + ev.data;
                setTranslated(incoming);
            };

            socket.send(JSON.stringify(request));
        }

    }, [socket]);

    useEffect(() => {
        const socket = new WebSocket(`${backend_url}/streamer2/v1/translate`);

        socket.addEventListener('open', () => {
            console.log('socket connected');
            setIsConnected(true);
        });

        socket.addEventListener('close', () => {
            console.log('socket disconnected');
            setIsConnected(false);
        });

        setSocket(socket);

        return () => {
            if (socket.readyState !== 3) {
                socket.close();
            }

            setSocket(undefined);
        }
    }, [backend_url]);

    return [isConnected, translated, requestTranslation]
}

export default useTranslator;
