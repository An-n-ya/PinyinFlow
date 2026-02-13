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
    onPlay?: (id: string) => void;
    onStop?: (id: string) => void;
}

export const MessageBubble = memo(function MessageBubble({ message }: MessageBubbleProps) {
    const from = message.sender === 'user' ? 'user' : 'assistant';
    const [isCopied, setIsCopied] = useState(false);

    const handleCopy = async () => {
        try {
            await navigator.clipboard.writeText(message.text);
            setIsCopied(true);
            setTimeout(() => setIsCopied(false), 2000);
        } catch (err) {
            console.error('Failed to copy text: ', err);
        }
    };

    return (
        <Message from={from}>
            <MessageContent className="flex flex-row">
                <AudioVisualizer isPlaying={message.isPlaying} />
                <MessageResponse>{message.text}</MessageResponse>
            </MessageContent>
            <MessageActions>
                <MessageAction
                    label={isCopied ? '已复制' : '复制'}
                    tooltip={isCopied ? '已复制!' : '复制文本'}
                    onClick={handleCopy}
                >
                    {isCopied ? <CheckIcon className="size-4" /> : <CopyIcon className="size-4" />}
                </MessageAction>
            </MessageActions>
        </Message>
    );
});

MessageBubble.displayName = 'MessageBubble';
