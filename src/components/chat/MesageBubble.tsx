import AudioVisualizer from '@/lib/AudioVisualizer';
import React, { Fragment, useEffect } from 'react';
import { Message, MessageContent, MessageResponse } from '../ai-elements/message';

interface MessageBubbleProps {
    message: MessageType;
    onPlay?: (id: number) => void;
    onStop?: (id: number) => void;
}
export function MessageBubble({ message, onPlay, onStop }: MessageBubbleProps) {
    const [from, setFrom] = React.useState<'user' | 'assistant'>('user');
    useEffect(() => {
        setFrom(message.sender === 'user' ? 'user' : 'assistant');
    }, [message]);

    return (
        <Fragment key={message.id}>
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
        </Fragment>
    );
}
