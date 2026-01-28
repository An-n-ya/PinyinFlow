import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import Stack from '@mui/material/Stack';
import TextField from '@mui/material/TextField';
import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import ListItemText from '@mui/material/ListItemText';

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
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");
  const [pinyin, setPinyin] = useState("");
  const [py_list, setPyList] = useState<InputSegment[]>([]);

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke("greet", { name }));
  }
  
  async function split(input: string) : Promise<string> {
    return await invoke("split", {input})
  }
  async function tone_command(input: string) : Promise<Tone> {
    return await invoke("tone", {input})
  }
  
  async function submit_pinyin() {
    const splits = await split(pinyin)
    const tone = await tone_command(splits)
    if (splits.trim().length == 0) return
    const seg: InputSegment = {
      raw: pinyin,
      splits, tone
    };
    py_list.push(seg)
    console.log(py_list)
    setPinyin('')
  }

  return (
    <main className="container">
      <Stack direction="row" spacing={2} sx={{
      justifyContent: "center"}}>
        <Stack sx={{width: 400}}>
          <TextField
            id="outlined-multiline-static"
            label="pinyin"
            multiline
            rows={4}
            sx={{width: "100%"}}
            onChange={(e) => {
              setPinyin(e.target.value.trim())
            }}
            onKeyDown={(e) => {
              if (e.key == 'Enter' || e.code == "Space") {
                submit_pinyin()
              }
            }}
            value={pinyin}
          />
          <List sx={{width: '100%', overflow: "auto", height: 800}}>
            {py_list.map((py, index) => (
              <ListItem key={`${py.raw}-${index}`}>
                <ListItemText primary={py.tone.py_styled} sx={{textAlign: 'end'}}/>
              </ListItem>
            )).reverse()} 
          </List>
        </Stack>
      </Stack>
    </main>
  );
}

export default App;
