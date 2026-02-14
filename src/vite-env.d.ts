/// <reference types="vite/client" />

interface MessageType {
    id: number;
    text: string;
    sender: 'user' | 'speaker' | 'ai';
    timestamp: number;
    date: string;
    tc: TimeComsumption | null;
    isPlaying?: boolean;
    // method signatures for the class in Chat.tsx
    add_tts_timestamp?: (timestamp: number) => MessageType;
    play_finished?: () => MessageType;
}

interface TimeComsumption {
    tts: number;
}

interface AutocompleteTextareaProps extends React.ComponentProps<'textarea'> {
    suggestion: string[];
}
