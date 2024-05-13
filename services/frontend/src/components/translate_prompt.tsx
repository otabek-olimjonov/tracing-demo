'use client'

import useTranslator from "@/hooks/translator"
import { FormEvent, useState } from "react";

export function TranslatePrompt() {
    const [isConnected, translated, requestTranslation] = useTranslator('ws://localhost');

    const [input, setInput] = useState('');
    const [requestedTranslate, setRequestedTranslate] = useState('');

    const onSubmit = async (event: FormEvent<HTMLFormElement>) => {
        event.preventDefault();

        const formData = new FormData(event.currentTarget);
        const translateInput = formData.get('translate_input');

        if (translateInput) {
            const translate = translateInput.toString();

            setInput('');
            setRequestedTranslate(translate);
            requestTranslation(translate);
        }
    }

    return (
        <div className="flex flex-col items-center justify-center space-y-4">
            <form onSubmit={onSubmit}>
                <input disabled={!isConnected} placeholder="Translate phrase" value={input} onChange={(ev) => setInput(ev.target.value)} type="text" name="translate_input" className="px-4 py-2 text-2xl text-gray-900 rounded-lg bg-slate-200" />
                <button type="submit" hidden></button>
            </form>

            { requestedTranslate.length > 0 && 
                <div className="flex flex-col grow bg-blue-300/5 p-4 rounded-lg max-w-4xl wrap">
                    {requestedTranslate}
                </div>
            }
            { translated.length > 0 && 
                <div className="flex flex-col grow bg-red-300/5 p-4 rounded-lg max-w-4xl wrap">
                    {translated}
                </div>
            }
        </div>
    )

}