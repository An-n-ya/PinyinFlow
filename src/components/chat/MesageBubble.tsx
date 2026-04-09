import AudioVisualizer from '@/lib/AudioVisualizer';
import { memo } from 'react';
import { Message, MessageContent, MessageResponse } from '../ai-elements/message';
import { MessageType } from './Chat';

interface MessageBubbleProps {
    message: MessageType;
    onPlay?: (id: number) => void;
    onStop?: (id: number) => void;
}

export const MessageBubble = memo(function MessageBubble({
    message,
}: MessageBubbleProps) {
    const from = message.sender === 'user' ? 'user' : 'assistant';

    return (
        <Message from={from}>
            <MessageContent className="flex flex-row">
                <AudioVisualizer isPlaying={message.isPlaying} />
                <MessageResponse>{message.text}</MessageResponse>
            </MessageContent>
            {/* <MessageActions>
                <MessageAction label="Copy">
                    <CopyIcon/>
                </MessageAction>
            </MessageActions> */}
        </Message>
    );
});

MessageBubble.displayName = 'MessageBubble';
