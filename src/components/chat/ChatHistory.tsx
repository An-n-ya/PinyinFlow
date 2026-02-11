import { useEffect, useRef } from 'react';
import { MessageBubble } from './MesageBubble';

import {
    Conversation,
    ConversationContent,
    ConversationScrollButton,
} from '@/components/ai-elements/conversation';

interface ChatHistoryProps {
    messages: MessageType[];
    onPlay?: (id: number) => void;
    onStop?: (id: number) => void;
    containerRef?: React.RefObject<HTMLDivElement | null>;
}

export function ChatHistory({ messages, containerRef: _containerRef }: ChatHistoryProps) {
    const bottomRef = useRef<HTMLDivElement>(null);
    useEffect(() => {
        bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [messages]);

    return (
        <Conversation className="size-full">
            <ConversationContent>
                {messages.map(msg => (
                    <MessageBubble key={msg.id} message={msg} />
                ))}
            </ConversationContent>
            <ConversationScrollButton />
        </Conversation>
    );
}