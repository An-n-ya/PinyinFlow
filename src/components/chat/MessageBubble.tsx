import AudioVisualizer from '@/lib/AudioVisualizer';
import { CheckIcon, CopyIcon } from 'lucide-react';
import { memo, useState } from 'react';
import {
    Message,
    MessageAction,
    MessageActions,
    MessageContent,
    MessageResponse,
} from '../ai-elements/message';

interface MessageBubbleProps {
    message: MessageType;
    onPlay?: (id: number) => void;
    onStop?: (id: number) => void;
}

export const MessageBubble = memo(function MessageBubble({
    message,
    onPlay: _onPlay,
    onStop: _onStop,
}: MessageBubbleProps) {
    const from = message.sender === 'user' ? 'user' : 'assistant';
    const [isCopied, setIsCopied] = useState(false);

    const handleCopy = async () => {
        try {
            await navigator.clipboard.writeText(message.text);
            setIsCopied(true);
            setTimeout(() => setIsCopied(false), 2000);
        } catch (error) {
            console.error('Failed to copy text:', error);
        }
    };

    return (
        <Message from={from}>
            <MessageContent className="flex flex-row">
                <AudioVisualizer isPlaying={message.isPlaying} />
                <MessageResponse>{message.text}</MessageResponse>
            </MessageContent>
            <MessageActions className="group-[.is-user]:ml-auto">
                <MessageAction
                    label={isCopied ? '已复制' : '复制'}
                    tooltip={isCopied ? '已复制' : '复制'}
                    onClick={handleCopy}
                >
                    {isCopied ? <CheckIcon className="size-4" /> : <CopyIcon className="size-4" />}
                </MessageAction>
            </MessageActions>
        </Message>
    );
});

MessageBubble.displayName = 'MessageBubble';
