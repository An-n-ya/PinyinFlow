import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useCallback, useEffect, useState } from 'react';
import { ChatHistory } from './ChatHistory';
import { InputArea } from './InputArea';
import { MessageType } from './MessageType';

async function play(id: string, input: string) {
    try {
        let revised_input = await invoke('proofread', { id, input });
        await invoke('play', { id, input: revised_input });
    } catch (error_msg) {
        console.error(error_msg);
    }
}

export default function Chat() {
    const [messages, setMessages] = useState<MessageType[]>(TEST_DATA);

    useEffect(() => {
        const unlistenPromise = listen<{ AudioPlayed: { id: string } }>('audio-played', event => {
            const finishedId = event.payload.AudioPlayed.id;
            console.info(`audio id${finishedId} played`);

            setMessages(prev => prev.map(m => (m.id === finishedId ? m.play_finished() : m)));
        });
        const unlistenTTSfinished = listen<{ TTSFinished: { id: string; timestamp: number } }>(
            'tts-finished',
            event => {
                const payload = event.payload.TTSFinished;
                const finishedId = payload.id;
                const timestamp = payload.timestamp;
                setMessages(prev =>
                    prev.map(m => (m.id === finishedId ? m.add_tts_timestamp(timestamp) : m))
                );
            }
        );

        return () => {
            unlistenPromise.then(unlisten => unlisten());
            unlistenTTSfinished.then(unlisten => unlisten());
        };
    }, []);

    const submit_pinyin = useCallback(async (pinyin: string) => {
        console.info(`handle message ${pinyin}`);
        const newMsg = MessageType.new_user(pinyin);
        setMessages(prev => [...prev, newMsg]);
        await play(newMsg.id, pinyin);
    }, []);

    return (
        <div className="flex h-screen flex-col">
            <ChatHistory messages={messages}></ChatHistory>
            <InputArea onSendMessage={submit_pinyin} />
        </div>
    );
}

const TEST_DATA: MessageType[] = [
    new MessageType({
        id: '1',
        text: '你好，这是一个测试',
        sender: 'user',
        date: '10:00',
        isPlaying: false,
    }),
    new MessageType({
        id: '2',
        text: '你好',
        sender: 'ai',
        date: '10:01',
        isPlaying: false,
    }),
];
