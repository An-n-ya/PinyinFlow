import { describe, expect, it } from 'vitest';
import { MessageType } from './MessageType';

describe('MessageType', () => {
    it('should create a new user message', () => {
        const msg = MessageType.new_user('Hello');
        expect(msg.text).toBe('Hello');
        expect(msg.sender).toBe('user');
        expect(msg.isPlaying).toBe(true);
        expect(msg.id).toBeDefined();
        expect(typeof msg.id).toBe('string');
    });

    it('should create a new ai message', () => {
        const msg = MessageType.new_chat_bot('Hi');
        expect(msg.text).toBe('Hi');
        expect(msg.sender).toBe('ai');
        expect(msg.isPlaying).toBe(true);
    });

    it('should add tts timestamp', () => {
        const msg = MessageType.new_user('Test');
        const timestamp = msg.timestamp + 100;
        const newMsg = msg.add_tts_timestamp(timestamp);

        expect(newMsg).not.toBe(msg); // Immutability check
        expect(newMsg.tc).toEqual({ tts: 100 });
        expect(newMsg.id).toBe(msg.id);
    });

    it('should mark play as finished', () => {
        const msg = MessageType.new_user('Test');
        expect(msg.isPlaying).toBe(true);

        const newMsg = msg.play_finished();
        expect(newMsg.isPlaying).toBe(false);
        expect(newMsg.id).toBe(msg.id);
    });
});
