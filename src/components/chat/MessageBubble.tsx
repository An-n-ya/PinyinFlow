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
}

export const MessageBubble = memo(function MessageBubble({ message }: MessageBubbleProps) {
    const from = message.sender === 'user' ? 'user' : 'assistant';
    const [isCopied, setIsCopied] = useState(false);

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
            <MessageActions className="group-[.is-user]:ml-auto">
                <MessageAction label="Copy" onClick={handleCopy}>
                    {isCopied ? <CheckIcon className="size-4" /> : <CopyIcon className="size-4" />}
                </MessageAction>
            </MessageActions>
        </Message>
    );
});

MessageBubble.displayName = 'MessageBubble';
