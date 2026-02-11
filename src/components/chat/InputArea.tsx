import {
    PromptInput,
    PromptInputButton,
    PromptInputFooter,
    PromptInputSubmit,
    PromptInputTextarea,
} from '@/components/ai-elements/prompt-input';
import { CuboidIcon } from 'lucide-react';
import { useState } from 'react';

interface InputAreaProps {
    onSendMessage: (text: string) => void;
}

export function InputArea({ onSendMessage }: InputAreaProps) {
    const [input, setInput] = useState('');

    const handleSend = () => {
        if (!input.trim()) return;

        onSendMessage(input);
        setInput('');
    };

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter' || e.code === 'Space') {
            e.preventDefault();
            handleSend();
        }
    };
    return (
        <>
            <PromptInput onSubmit={handleSend} className="mt-4 w-full max-w-2xl mx-auto relative">
                <PromptInputTextarea
                    value={input}
                    placeholder=""
                    onChange={e => setInput(e.target.value)}
                    onKeyDown={handleKeyDown}
                    className="pr-12 h-40"
                />
                <PromptInputFooter>
                    <PromptInputButton>
                        <CuboidIcon />
                        <span>模型</span>
                    </PromptInputButton>
                    <PromptInputSubmit
                        status="ready"
                        disabled={!input.trim()}
                        className="absolute bottom-1 right-1"
                    />
                </PromptInputFooter>
            </PromptInput>
        </>
    );
}
