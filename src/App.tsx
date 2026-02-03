import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Stack from '@mui/material/Stack';
import {info, error} from '@tauri-apps/plugin-log';
import AppHeader from "./ui/Header";
import { InputArea } from "./ui/InputArea";
import { ChatHistory } from "./ui/ChatHistory";

interface InputSegment {
  raw: string;
  splits: string;
  tone: Tone;
}
interface Tone {
  tone: string;
  pinyin: string;
  py_styled: string;
}


function App() {
  const [messages, setMessages] = useState<Message[]>([
    {
      id: 1,
      text: "你好，这是一个测试",
      sender: "user",
      timestamp: "10:00",
      isPlaying: false
    },
    {
      id: 2,
      text: "ahjfklahdsjfhajlkdsh",
      sender: "ai",
      timestamp: "10:01",
      isPlaying: false
    },
    {
      id: 2,
      text: "ahjfklahdsjfhajlkdsh",
      sender: "ai",
      timestamp: "10:01",
      isPlaying: false
    },
    {
      id: 2,
      text: "ahjfklahdsjfhajlkdsh",
      sender: "ai",
      timestamp: "10:01",
      isPlaying: false
    },
    {
      id: 2,
      text: "ahjfklahdsjfhajlkdsh",
      sender: "ai",
      timestamp: "10:01",
      isPlaying: false
    },
    {
      id: 2,
      text: "ahjfklahdsjfhajlkdsh",
      sender: "ai",
      timestamp: "10:01",
      isPlaying: false
    },
    {
      id: 2,
      text: "ahjfklahdsjfhajlkdsh",
      sender: "ai",
      timestamp: "10:01",
      isPlaying: false
    },
    {
      id: 2,
      text: "ahjfklahdsjfhajlkdsh",
      sender: "ai",
      timestamp: "10:01",
      isPlaying: false
    },
    {
      id: 2,
      text: "ahjfklahdsjfhajlkdsh",
      sender: "ai",
      timestamp: "10:01",
      isPlaying: false
    },
    {
      id: 2,
      text: "ahjfklahdsjfhajlkdsh",
      sender: "ai",
      timestamp: "10:01",
      isPlaying: false
    },
    {
      id: 1,
      text: "你好，这是一个测试",
      sender: "user",
      timestamp: "10:00",
      isPlaying: false
    },
    {
      id: 1,
      text: "你好，这是一个测试",
      sender: "user",
      timestamp: "10:00",
      isPlaying: false
    },
    {
      id: 1,
      text: "你好，这是一个测试",
      sender: "user",
      timestamp: "10:00",
      isPlaying: false
    },
    {
      id: 1,
      text: "你好，这是一个测试",
      sender: "user",
      timestamp: "10:00",
      isPlaying: false
    },
  ]);
  const [py_list, setPyList] = useState<InputSegment[]>([]);
  
  async function split(input: string) : Promise<string> {
    return await invoke("split", {input})
  }
  async function tone_command(input: string) : Promise<Tone> {
    return await invoke("tone", {input})
  }
  async function play(input: string) {
    try { 
      await invoke("play", {input})
    } catch(error_msg) {

      error(`${error_msg}`);
      console.error(error_msg)
    }
  }
  
  async function submit_pinyin(pinyin: string) {
    // const splits = await split(pinyin)
    // const tone = await tone_command(splits)
    // if (splits.trim().length == 0) return
    // const seg: InputSegment = {
    //   raw: pinyin,
    //   splits, tone
    // };
    // //FIXME: is this reactive?
    // py_list.push(seg)
    //info(`[js] submit pinyin {pinyin}`)
    // play(tone.pinyin)
    const newMsg: Message = {
      id: Date.now(),
      text: pinyin,
      sender: "user",
      timestamp: new Date().toLocaleDateString([], {hour: '2-digit', minute: '2-digit'}) 
    }
    setMessages(prev => [...prev, newMsg])
    play(pinyin)
  }

  return (
    <main className="container">
      <AppHeader/>
      <Stack direction="row" spacing={2} sx={{
        justifyContent: "center",
        overflow: "hidden",
        height: "100%"
      }}>
        <Stack sx={{height: '100%', width: '100%', alignItems: 'center'}}>
          <ChatHistory messages={messages}></ChatHistory>
          <InputArea onSendMessage={submit_pinyin}/>
        </Stack>
      </Stack>
    </main>
  );
}

export default App;
