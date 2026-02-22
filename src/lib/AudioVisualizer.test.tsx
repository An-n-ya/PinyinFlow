import { renderToStaticMarkup } from 'react-dom/server';
import { describe, expect, it } from 'vitest';
import AudioVisualizer from './AudioVisualizer';

describe('AudioVisualizer', () => {
    it('has role="img" and aria-label', () => {
        const html = renderToStaticMarkup(<AudioVisualizer isPlaying={true} />);
        expect(html).toContain('role="img"');
        expect(html).toContain('aria-label="正在播放音频"');
    });
});
