import { Box, ListItem, ListItemText } from "@mui/material";
import {styled} from "@mui/material/styles";
import React from "react";
import { useEffect } from "react";

const BubbleListItem = styled(ListItem)({
    borderRadius: 5,
    justifyContent: 'var(--justify-content)'
})

const aiVars = {
    '--justify-content': 'flex-start'
} as React.CSSProperties
const userVars = {
    '--justify-content': 'flex-end'
} as React.CSSProperties

interface MessageBubbleProps {
    message: Message;
    onPlay?: (id: number) => void;
    onStop?: (id: number) => void;
}
export function MessageBubble({message, onPlay, onStop}: MessageBubbleProps) {
    const [vars, setVars] = React.useState<React.CSSProperties>(userVars);
    useEffect(() => {
        setVars(message.sender === 'user' ? userVars : aiVars);
    }, [message])
    
    return (
        <BubbleListItem style={vars} key={message.id}>
            <ListItemText primary={message.text} sx={{
                flex: "0 0 auto",
                backgroundColor: "rgba(0,0,0,0.04)",
                borderRadius: '12px',
                padding: "9px 16px 9px 16px"
                }}/>
        </BubbleListItem>
    )
}