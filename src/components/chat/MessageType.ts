export class MessageType {
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
