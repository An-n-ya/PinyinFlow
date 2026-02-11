import AudioVisualizer from '@/lib/AudioVisualizer';
import React, { memo } from 'react';
import { Message, MessageContent, MessageResponse } from '../ai-elements/message';

interface MessageBubbleProps {
    message: MessageType;
    onPlay?: (id: number) => void;
    onStop?: (id: number) => void;
}

export const MessageBubble = memo(function MessageBubble({
    message,
    onPlay,
    onStop,
}: MessageBubbleProps) {
    const from = message.sender === 'user' ? 'user' : 'assistant';

    return (
        <Message from={from}>
            <MessageContent className="flex-row">
                {message.isPlaying && <AudioVisualizer />}
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
