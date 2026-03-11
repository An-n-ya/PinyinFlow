import AudioVisualizer from '@/lib/AudioVisualizer';
import { Check, Copy } from 'lucide-react';
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
    const [isCopied, setIsCopied] = useState(false);
    const from = message.sender === 'user' ? 'user' : 'assistant';

    const handleCopy = () => {
        navigator.clipboard.writeText(message.text);
        setIsCopied(true);
        setTimeout(() => setIsCopied(false), 2000);
    };

    return (
        <Message from={from}>
            <MessageContent className="flex flex-row">
                <AudioVisualizer isPlaying={message.isPlaying} />
                <MessageResponse>{message.text}</MessageResponse>
            </MessageContent>
            <MessageActions className={from === 'user' ? 'ml-auto' : ''}>
                <MessageAction
                    label={isCopied ? 'Copied' : 'Copy message'}
                    tooltip={isCopied ? 'Copied!' : 'Copy'}
                    onClick={handleCopy}
                >
                    {isCopied ? <Check size={16} /> : <Copy size={16} />}
                </MessageAction>
            </MessageActions>
        </Message>
    );
});

MessageBubble.displayName = 'MessageBubble';
