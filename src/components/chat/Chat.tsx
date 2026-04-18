import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useCallback, useEffect, useState } from 'react';
import { ChatHistory } from './ChatHistory';
import { InputArea } from './InputArea';

class MessageType {
    id: string = crypto.randomUUID();
    text: string = '';
    sender: 'user' | 'ai' = 'user';
    date: string = new Date().toLocaleDateString([], { hour: '2-digit', minute: '2-digit' });
    timestamp: number = Date.now();
    tc: TimeComsumption | null = null;
    isPlaying?: boolean = false;
    constructor(config?: Partial<MessageType>) {
        // 使用 Object.assign 将配置合并到实例中
        Object.assign(this, config);
    }
    static new_user(text: string): MessageType {
        const msg = new MessageType();
        msg.text = text;
        msg.isPlaying = true;
        return msg;
    }
    static new_chat_bot(text: string): MessageType {
        const msg = MessageType.new_user(text);
        msg.sender = 'ai';
        return msg;
    }
    add_tts_timestamp(timestamp: number): MessageType {
        this.tc = { tts: timestamp - this.timestamp };
        return new MessageType({
            ...this,
            tc: this.tc,
        });
    }
    play_finished(): MessageType {
        return new MessageType({
            ...this,
            isPlaying: false,
        });
    }
}

export default function Chat() {
    const [messages, setMessages] = useState<MessageType[]>(TEST_DATA);

    const play = useCallback(async (id: string, input: string) => {
        try {
            let revised_input = await invoke('proofread', { id, input });
            await invoke('play', { id, input: revised_input });
        } catch (error_msg) {
            console.error(error_msg);
        }
    }, []);

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

    const submit_pinyin = useCallback(
        async (pinyin: string) => {
            console.info(`handle message ${pinyin}`);
            const newMsg = MessageType.new_user(pinyin);
            setMessages(prev => [...prev, newMsg]);
            play(newMsg.id, pinyin);
        },
        [play]
    );

    return (
        <div className="flex flex-1 flex-col bg-slate-50">
            <div className="mx-auto flex size-full max-w-md flex-1 flex-col">
                <ChatHistory messages={messages}></ChatHistory>
                <InputArea onSendMessage={submit_pinyin} />
            </div>
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
