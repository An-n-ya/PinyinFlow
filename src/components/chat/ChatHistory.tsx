import { MessageBubble } from './MesageBubble';
import { MessageType } from './Chat';

import {
    Conversation,
    ConversationContent,
    ConversationScrollButton,
} from '@/components/ai-elements/conversation';

interface ChatHistoryProps {
    messages: MessageType[];
    onPlay?: (id: number) => void;
    onStop?: (id: number) => void;
}

export function ChatHistory({ messages }: ChatHistoryProps) {
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
