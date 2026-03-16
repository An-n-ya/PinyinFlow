import { MessageBubble } from './MesageBubble';

import {
    Conversation,
    ConversationContent,
    ConversationScrollButton,
} from '@/components/ai-elements/conversation';

interface ChatHistoryProps {
    messages: MessageType[];
    onPlay?: (id: string) => void;
    onStop?: (id: string) => void;
}

export function ChatHistory({ messages, onPlay, onStop }: ChatHistoryProps) {
    void onPlay;
    void onStop;
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
