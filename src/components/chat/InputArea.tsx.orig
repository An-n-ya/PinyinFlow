import {
    PromptInput,
    PromptInputButton,
    PromptInputFooter,
    PromptInputSubmit,
    PromptInputTextarea,
} from '@/components/ai-elements/prompt-input';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import { getAppState } from '@/lib/store';
import { Channel, invoke } from '@tauri-apps/api/core';
import { CuboidIcon } from 'lucide-react';
import { useRef, useState } from 'react';

interface InputAreaProps {
    onSendMessage: (text: string) => void;
}

type ReplyCompleteEvent =
    | {
          event: 'finished';
          data: {};
      }
    | { event: 'content'; data: string };

const notTypingKey = [
    'Escape',
    'Tab',
    'ArrowLeft',
    'ArrowRight',
    'ArrowUp',
    'ArrowDown',
    'Backspace',
    'Enter',
    'Super',
    'Delete',
];
export function InputArea({ onSendMessage }: InputAreaProps) {
    const [input, setInput] = useState('');
    const [suggestion, setSuggestion] = useState([] as string[]);
    const timeoutRef = useRef<NodeJS.Timeout>(null);

    const handleSend = () => {
        if (!input.trim()) return;

        setSuggestion([]);
        onSendMessage(input);
        setInput('');
    };

    const handleKeyDown = (e: React.KeyboardEvent<HTMLTextAreaElement>) => {
        if (e.key === 'Tab' && suggestion) {
            e.preventDefault();
            setInput(input + suggestion.join(''));
            setSuggestion([]);
            timeoutRef.current?.close();
            return;
        }
        // clear suggestion
        setSuggestion([]);

        if (timeoutRef.current) clearTimeout(timeoutRef.current);
        if (
            getAppState().pref.enableCompleteInput &&
            input.length > 0 &&
            !notTypingKey.find(value => value === e.key)
        ) {
            console.info(`setting timeout key:${e.key}`);
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
                    input: input,
                    onEvent,
                });
            }, 500);
        }
    };

    return (
        <>
            <PromptInput onSubmit={handleSend} className="my-4 w-full max-w-2xl flex-none">
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
}
