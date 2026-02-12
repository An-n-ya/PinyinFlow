import { MessageBubble } from './MessageBubble';

import {
    Conversation,
    ConversationContent,
    ConversationScrollButton,
} from '@/components/ai-elements/conversation';

interface ChatHistoryProps {
    messages: MessageType[];
    onReplay?: (id: string, text: string) => void;
}

export function ChatHistory({ messages, onReplay }: ChatHistoryProps) {
    return (
        <Conversation className="size-full">
            <ConversationContent>
                {messages.map(msg => (
                    <MessageBubble key={msg.id} message={msg} onReplay={onReplay} />
                ))}
            </ConversationContent>
            <ConversationScrollButton />
        </Conversation>
    );
}
