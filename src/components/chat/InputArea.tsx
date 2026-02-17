import {
    PromptInput,
    PromptInputButton,
    PromptInputFooter,
    PromptInputSubmit,
    PromptInputTextarea,
} from '@/components/ai-elements/prompt-input';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import { Channel, invoke } from '@tauri-apps/api/core';
import { CuboidIcon } from 'lucide-react';
import { type KeyboardEvent, memo, useRef, useState } from 'react';

interface InputAreaProps {
    onSendMessage: (text: string) => void;
}

type ReplyCompleteEvent =
    | {
          event: 'finished';
          data: {};
      }
    | { event: 'content'; data: string };

export const InputArea = memo(function InputArea({ onSendMessage }: InputAreaProps) {
    const [input, setInput] = useState('');
    const [suggestion, setSuggestion] = useState(['a', 'b', 'c'] as string[]);
    const timeoutRef = useRef<ReturnType<typeof setTimeout>>(null);

    const handleSend = () => {
        if (!input.trim()) return;

        setSuggestion([]);
        onSendMessage(input);
        setInput('');
    };

    const handleKeyDown = (e: KeyboardEvent<HTMLTextAreaElement>) => {
        if (e.key === 'Tab' && suggestion) {
            e.preventDefault();
            setInput(input + suggestion.join(''));
            setSuggestion([]);
            timeoutRef.current?.close();
            return;
        }
        // clear suggestion
        setSuggestion([]);

        if (e.key !== 'Escape') {
            if (timeoutRef.current) clearTimeout(timeoutRef.current);
            timeoutRef.current = setTimeout(async () => {
                const onEvent = new Channel<ReplyCompleteEvent>();
                onEvent.onmessage = message => {
                    if (message.event === 'content') {
                        setSuggestion(prev => prev.concat(message.data));
                    } else if (message.event === 'finished') {
                        return;
                    }
                };

                await invoke('complete_message', {
                    input: '你是谁',
                    onEvent,
                });
            }, 500);
        }
    };

    return (
        <>
            <PromptInput onSubmit={handleSend} className="relative mx-auto mt-4 w-full max-w-2xl">
                <PromptInputTextarea
                    suggestion={suggestion}
                    value={input}
                    placeholder="请输入拼音..."
                    onChange={e => setInput(e.target.value)}
                    onKeyDown={handleKeyDown}
                    className="h-40 pr-12"
                />
                <PromptInputFooter>
                    <PromptInputButton tooltip="选择模型">
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
});
