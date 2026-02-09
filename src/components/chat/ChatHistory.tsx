import { Container } from '@mui/material';
import List from '@mui/material/List';
import { useEffect, useRef } from 'react';
import { MessageBubble } from './MesageBubble';

interface ChatHistoryProps {
    messages: Message[];
    onPlay?: (id: number) => void;
    onStop?: (id: number) => void;
    containerRef?: React.RefObject<HTMLDivElement | null>;
}

export function ChatHistory({ messages, containerRef }: ChatHistoryProps) {
    const bottomRef = useRef<HTMLDivElement>(null);
    useEffect(() => {
        bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [messages]);

    return (
        <Container id="ChatHistoryContainer" ref={containerRef}>
            <List sx={{ width: 'min(800px, 90%)' }}>
                {messages.map(msg => (
                    <MessageBubble message={msg} />
                ))}
            </List>
            <div ref={bottomRef}></div>
        </Container>
    );
}
