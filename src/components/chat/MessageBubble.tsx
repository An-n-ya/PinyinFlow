import {
    Message,
    MessageAction,
    MessageActions,
    MessageContent,
    MessageResponse,
} from '@/components/ai-elements/message';
import AudioVisualizer from '@/lib/AudioVisualizer';
import { PlayIcon } from 'lucide-react';
import { memo } from 'react';

interface MessageBubbleProps {
    message: MessageType;
    onReplay?: (id: string, text: string) => void;
}

export const MessageBubble = memo(function MessageBubble({
    message,
    onReplay,
}: MessageBubbleProps) {
    const from = message.sender === 'user' ? 'user' : 'assistant';

    return (
        <Message from={from}>
            <MessageContent className="flex-row">
                {message.isPlaying && <AudioVisualizer />}
                <MessageResponse>{message.text}</MessageResponse>
            </MessageContent>
            <MessageActions className={from === 'user' ? 'justify-end' : ''}>
                <MessageAction
                    tooltip="Replay message"
                    onClick={() => onReplay?.(message.id, message.text)}
                    disabled={message.isPlaying}
                    aria-label="Replay message"
                >
                    <PlayIcon className="size-4" />
                </MessageAction>
            </MessageActions>
        </Message>
    );
});

MessageBubble.displayName = 'MessageBubble';
