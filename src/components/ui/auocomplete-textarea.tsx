import { motion } from 'motion/react';
import { useRef } from 'react';

import { cn } from '@/lib/utils';

const AutocompleteTextarea = ({ className, suggestion, ...props }: AutocompleteTextareaProps) => {
    const textareaRef = useRef<HTMLTextAreaElement>(null);

    // const fullResponse = '这是一个模拟的流式响应...\n它可以分段显示文字，\n就像真实的 AI 一样。';

    // const startSimulation = () => {
    //     setSuggestion([]);
    //     setIsStreaming(true);
    //     if (streamIntervalRef.current) clearInterval(streamIntervalRef.current);

    //     let charIndex = 0;

    //     streamIntervalRef.current = setInterval(() => {
    //         if (charIndex < fullResponse.length) {
    //             const nextChar = fullResponse[charIndex];

    //             setSuggestion(prev => prev.concat([nextChar]));
    //             charIndex++;
    //         } else {
    //             stopSimulation();
    //         }
    //     }, 100);
    // };

    // const stopSimulation = () => {
    //     if (streamIntervalRef.current) {
    //         clearInterval(streamIntervalRef.current);
    //         streamIntervalRef.current = null;
    //     }
    //     setIsStreaming(false);
    // };

    // // 组件卸载时清理定时器
    // useEffect(() => {
    //     return () => stopSimulation();
    // }, []);

    // const handleInputWrap = e => {
    //     const value = e.target.value;
    //     setInput(value);

    //     if (value.endsWith('如何')) {
    //         setSuggestion(
    //             '快速实现补全功能这是一个恨恨恨恶化讷河蔫儿坏蔫儿坏嗯嗯别长的字符串，它换行是否正确？？'
    //         );
    //     } else {
    //         setSuggestion([]);
    //     }

    //     onInput(e);
    // };

    // const handleKeyDownWrap = e => {
    //     if (e.key === 'Tab' && suggestion) {
    //         e.preventDefault();
    //         setInput(input + suggestion);
    //         setSuggestion('');
    //     }
    //     onKeyDown(e);
    // };

    // FIXME: sync overlay to textarea on scroll
    // const handleScroll = e => {
    //     if (overlayRef.current) {
    //         overlayRef.current.scrollTop = e.target.scrollTop;
    //     }
    //     const handleScroll = e => {
    //         if (overlayRef.current) {
    //             overlayRef.current.scrollTop = e.target.scrollTop;
    //         }
    //     };
    // };

    return (
        <div className="relative w-full">
            <div
                className={cn(
                    'pointer-events-none absolute inset-0 border border-transparent px-3 py-2 md:text-sm',
                    className
                )}
                aria-hidden="true"
            >
                <span className="text-transparent">{textareaRef.current?.value}</span>
                {suggestion.map((item, index) => (
                    <motion.span
                        className="inline-block text-gray-400"
                        initial={{ opacity: 0, y: 5, x: 5 }}
                        animate={{ opacity: 1, y: 0, x: 0 }}
                        transition={{ duration: 0.1 }}
                        key={`${item}-${index}`}
                    >
                        {item}
                    </motion.span>
                ))}
            </div>

            <textarea
                ref={textareaRef}
                data-slot="textarea"
                className={cn(
                    'border-input placeholder:text-muted-foreground aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:bg-input/30 flex field-sizing-content min-h-16 w-full resize-none rounded-md border border-transparent bg-transparent px-3 py-2 text-base shadow-xs shadow-transparent transition-[color,box-shadow] outline-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm',
                    className
                )}
                {...props}
            />
        </div>
    );
};

export default AutocompleteTextarea;
