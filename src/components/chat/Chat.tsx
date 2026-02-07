import { useState, useEffect } from "react";
import Stack from '@mui/material/Stack';
import { InputArea } from "./InputArea";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ChatHistory } from "./ChatHistory";

class Message {
  id: number = Date.now() % 2147483647; // FIXME: switch to uuid
  text: string = "";
  sender: "user" | "ai" = "user";
  date: string = new Date().toLocaleDateString([], {hour: '2-digit', minute: '2-digit'});
  timestamp: number = Date.now();
  tc: TimeComsumption | null = null;
  isPlaying?: boolean = false;
  constructor(config?: Partial<Message>) {
    // 使用 Object.assign 将配置合并到实例中
    Object.assign(this, config);
  }
  static new_user(text: string): Message {
    const msg =  new Message();
    msg.text = text;
    msg.isPlaying = true;
    return msg;
  }
  static new_chat_bot(text: string): Message {
    const msg = Message.new_user(text);
    msg.sender = "ai";
    return msg;
  }
  add_tts_timestamp(timestamp: number): Message {
    this.tc = {tts: timestamp - this.timestamp}
    return new Message({
      ...this,
      tc: this.tc
    });
  }
  play_finished(): Message {
    return new Message({
      ...this,
      isPlaying: false
    });
  }
}


export default function Chat() {
  const [messages, setMessages] = useState<Message[]>(TEST_DATA);
  async function play(id: number, input: string) {
    try { 
      await invoke("play", {id, input})
    } catch(error_msg) {
      console.error(error_msg)
    }
  }
  
  useEffect(() => {
    const unlistenPromise = listen<{AudioPlayed:{ id: number }}>("audio-played", (event) => {
      const finishedId = event.payload.AudioPlayed.id;
      console.info(`audio id${finishedId} played`);
  
      setMessages((prev) =>
        prev.map((m) =>
          m.id === finishedId
            ? m.play_finished()
            : m
        )
      );
    });
    const unlistenTTSfinished = listen<{TTSFinished:{ id: number, timestamp: number}}>("tts-finished", (event) => {
      const payload = event.payload.TTSFinished;
      const finishedId = payload.id;
      const timestamp = payload.timestamp;
      setMessages((prev) =>
        prev.map((m) =>
          m.id === finishedId
            ? m.add_tts_timestamp(timestamp)
            : m
        )
      );
    });
  
    return () => {
      unlistenPromise.then((unlisten) => unlisten());
      unlistenTTSfinished.then((unlisten) => unlisten());
    };
  }, []);
  
  async function submit_pinyin(pinyin: string) {
    console.info(`handle message ${pinyin}`);
    const newMsg = Message.new_user(pinyin);
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
      date: "10:00",
      isPlaying: false
    },
    {
      id: 2,
      text: "你好",
      sender: "ai",
      date: "10:01",
      isPlaying: false
    }
]