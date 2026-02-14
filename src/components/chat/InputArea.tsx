import {
    PromptInput,
    PromptInputButton,
    PromptInputFooter,
    PromptInputSubmit,
    PromptInputTextarea,
} from '@/components/ai-elements/prompt-input';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
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

    return (
        <>
            <PromptInput onSubmit={handleSend} className="relative mx-auto mt-4 w-full max-w-2xl">
                <PromptInputTextarea
                    value={input}
                    placeholder="请输入拼音..."
                    aria-label="输入拼音"
                    onChange={e => setInput(e.target.value)}
                    className="h-40 pr-12"
                />
                <PromptInputFooter>
                    <PromptInputButton
                        tooltip="模型选择暂不可用"
                        className="opacity-50 cursor-not-allowed"
                    >
                        <CuboidIcon />
                        <span>模型</span>
                    </PromptInputButton>
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <PromptInputSubmit
                                status="ready"
                                disabled={!input.trim()}
                                className="absolute right-1 bottom-1"
                                aria-label="发送"
                            />
                        </TooltipTrigger>
                        <TooltipContent>发送</TooltipContent>
                    </Tooltip>
                </PromptInputFooter>
            </PromptInput>
        </>
    );
}
