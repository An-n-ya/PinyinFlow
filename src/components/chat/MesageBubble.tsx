import { Box, Button, Card, CardActions, CardContent, ListItem, ListItemText, Stack, Typography, CircularProgress } from "@mui/material";
import {styled} from "@mui/material/styles";
import React from "react";
import { useEffect } from "react";
import AccessTimeIcon from '@mui/icons-material/AccessTime';


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
        <Card key={message.id}>
            <CardContent sx={{ padding: "12px !important" }}>
                <Typography variant="body1" component="div">
                    {message.text}
                </Typography>
                    <Stack direction="row">
                        {message.tc &&  (<>
                                <AccessTimeIcon fontSize="small" sx={{
                                    height: "1.5rem", color: 'text.secondary',
                                    marginRight: "0.5rem",
                                }}/>
                                <Typography variant="body2" component="div" sx={{color: "text.secondary"}}>
                                    {(message.tc.tts) + "ms"}
                                </Typography>
                            </>
                            )
                        }
                        {/* <Typography sx={{ color: 'text.secondary'}}>
                            {message.date}
                        </Typography> */}
                        {message.isPlaying && (
                            <CircularProgress size={16} sx={{ ml: 1 }}/>
                        )}
                    </Stack>
            </CardContent>
        </Card>
        </BubbleListItem>
    )
}