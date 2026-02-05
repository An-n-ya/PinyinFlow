import { TextField } from "@mui/material";
import { useState } from "react";

interface InputAreaProps {
    onSendMessage: (text: string) => void;
}

export function InputArea({onSendMessage}: InputAreaProps) {
    const [input, setInput] = useState("");
    
    const handleSend = () => {
        if (!input.trim()) return;
        
        onSendMessage(input);
        setInput("")
    }

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (
            e.key === "Enter" || e.code === "Space"
        ) {
            e.preventDefault();
            handleSend();
        }
    }
    return (
        <TextField
            hiddenLabel
            id="outlined-multiline-static"
            multiline
            rows={4}
            sx={{
                margin: "10px",
                width: "min(800px, 90%)",
                borderRadius: "20px",
                boxShadow:"rgba(0, 0, 0, 0) 0px 0px 0px 0px, rgba(0, 0, 0, 0) 0px 0px 0px 0px, rgba(0, 0, 0, 0) 0px 0px 0px 0px, rgba(0, 0, 0, 0) 0px 0px 0px 0px, rgba(0, 0, 0, 0.02) 0px 2px 4px 0px, rgba(0, 0, 0, 0.04) 0px 4px 16px 0px, rgba(0, 0, 0, 0.08) 0px 8px 32px 0px",
                '& .MuiInputBase-root': {
                    borderRadius: "20px"
                }
            }}
            onChange={(e) => {
                setInput(e.target.value)
            }}
            onKeyDown={handleKeyDown}
            value={input}
            />
    )
}