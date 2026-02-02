import List from '@mui/material/List';
import { useEffect } from "react";
import { useRef } from "react";
import { MessageBubble } from './MesageBubble';
import {css} from "@emotion/react"

interface ChatHistoryProps {
    messages: Message[];
    onPlay?: (id: number) => void;
    onStop?: (id: number) => void;
}
export function ChatHistory({messages, onPlay, onStop}: ChatHistoryProps) {
    const buttomRef = useRef<HTMLDivElement>(null);    
    useEffect(() => {
        buttomRef.current?.scrollIntoView({behavior: "smooth"});
    }, [messages])
    

    return (
        <div id="ChatHistoryContainer">
            <List sx={{width: "800px"}}>
                {
                    messages.map((msg) => (
                        <MessageBubble message={msg}/>      
                    )
                )
                }
            </List>
            <div ref={buttomRef}></div>
        </div>
    )
}