/// <reference types="vite/client" />

interface Message {
  id: number;
  text: string;
  sender: "user" | "ai";
  timestamp: number;
  date: string,
  tc: TimeComsumption | null;
  isPlaying?: boolean;
}

interface TimeComsumption {
  tts: number
}