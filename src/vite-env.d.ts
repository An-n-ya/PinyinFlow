/// <reference types="vite/client" />

interface MessageType {
    id: number;
    text: string;
    sender: 'user' | 'speaker';
    timestamp: number;
    date: string;
    tc: TimeComsumption | null;
    isPlaying?: boolean;
}

interface TimeComsumption {
    tts: number;
}
