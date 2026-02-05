import { useRef, useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Stack from '@mui/material/Stack';
import {info, error} from '@tauri-apps/plugin-log';
import AppHeader from "./ui/Header";
import { InputArea } from "./ui/InputArea";
import { ChatHistory } from "./ui/ChatHistory";
import { ThemeProvider, createTheme } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
const darkTheme = createTheme({
  palette: {
    mode: 'dark',
  },
});


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
  const [messages, setMessages] = useState<Message[]>(TEST_DATA);
  const [py_list, setPyList] = useState<InputSegment[]>([]);
  const chatContainerRef = useRef<HTMLDivElement>(null);
  
  async function split(input: string) : Promise<string> {
    return await invoke("split", {input})
  }
  async function tone_command(input: string) : Promise<Tone> {
    return await invoke("tone", {input})
  }
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
  
    // 组件卸载时取消监听，防止内存泄漏
    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);
  
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
    <ThemeProvider theme={darkTheme}>
    <CssBaseline/>
    <main className="container">
      <AppHeader scrollTarget={chatContainerRef}/>
      <Stack direction="row" spacing={2} sx={{
        justifyContent: "center",
        overflow: "hidden",
        height: "100vh",
      }}>
        <Stack sx={{height: '100%', width: '100%', alignItems: 'center'}}>
          <ChatHistory messages={messages} containerRef={chatContainerRef}></ChatHistory>
          <InputArea onSendMessage={submit_pinyin}/>
        </Stack>
      </Stack>
    </main>
    </ThemeProvider>
  );
}

export default App;


const TEST_DATA = [
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
      text: "你好，这是一个动画测试",
      sender: "user",
      timestamp: "10:00",
      isPlaying: true
    },
  ]