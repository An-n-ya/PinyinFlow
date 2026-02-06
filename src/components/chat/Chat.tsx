import { useState, useEffect } from "react";
import Stack from '@mui/material/Stack';
import { InputArea } from "./InputArea";
import { error } from '@tauri-apps/plugin-log';
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ChatHistory } from "./ChatHistory";


export default function Chat() {
  const [messages, setMessages] = useState<Message[]>(TEST_DATA);
  async function play(id: number, input: string) {
    try { 
      await invoke("play", {id, input})
    } catch(error_msg) {

      error(`${error_msg}`);
      console.error(error_msg)
    }
  }
  
  useEffect(() => {
    const unlistenPromise = listen<{ id: number }>("audio-played", (event) => {
      console.log(event);
      const finishedId = event.payload.id;
  
      setMessages((prev) =>
        prev.map((m) =>
          m.id === finishedId ? { ...m, isPlaying: false } : m
        )
      );
    });
  
    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);
  
  async function submit_pinyin(pinyin: string) {
    console.info(`handle message ${pinyin}`);
    const newMsg: Message = {
      id: Date.now() % 2147483647, // FIXME: switch to uuid
      text: pinyin,
      sender: "user",
      timestamp: new Date().toLocaleDateString([], {hour: '2-digit', minute: '2-digit'}),
      isPlaying: true
    }
    setMessages(prev => [...prev, newMsg])
    play(newMsg.id, pinyin)
  }
    return (
      <Stack direction="row" spacing={2} sx={{
        justifyContent: "center",
        overflow: "hidden",
        height: "100vh",
      }}>
        <Stack sx={{height: '100%', width: '100%', alignItems: 'center'}}>
          <ChatHistory messages={messages}></ChatHistory>
          <InputArea onSendMessage={submit_pinyin}/>
        </Stack>
      </Stack>
    )
}

const TEST_DATA: Message[]  = [
    {
      id: 1,
      text: "你好，这是一个测试",
      sender: "user",
      timestamp: "10:00",
      isPlaying: false
    },
    {
      id: 2,
      text: "你好",
      sender: "ai",
      timestamp: "10:01",
      isPlaying: false
    }
]