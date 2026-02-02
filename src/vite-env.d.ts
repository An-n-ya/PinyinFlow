/// <reference types="vite/client" />

interface Message {
  id: number;
  text: string;
  sender: "user" | "ai";
  timestamp: string;
  isPlaying?: boolean;
}