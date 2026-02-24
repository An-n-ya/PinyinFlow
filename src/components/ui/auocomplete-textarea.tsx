import { motion } from 'motion/react';
import { useEffect, useRef } from 'react';

import { cn } from '@/lib/utils';

const clamp = (num: number, min: number, max: number) => Math.min(Math.max(num, min), max);

const AutocompleteTextarea = ({ className, suggestion, ...props }: AutocompleteTextareaProps) => {
    const textareaRef = useRef<HTMLTextAreaElement>(null);
    const suggestionLayerRef = useRef<HTMLDivElement>(null);

    const syncTextareaHeight = () => {
        if (!textareaRef.current || !suggestionLayerRef.current) return;

        const layerHeight = suggestionLayerRef.current.offsetHeight;

        textareaRef.current.style.height = `${clamp(layerHeight, 64, 256)}px`; // max-h-64 = 256px min-h-16 = 64px
    };

    useEffect(() => {
        const textarea = textareaRef.current;
        if (!textarea) return;
        syncTextareaHeight();
        const handleInput = () => syncTextareaHeight();
        textarea.addEventListener('input', handleInput);
        window.addEventListener('resize', handleInput);
        return () => {
            textarea.removeEventListener('input', handleInput);
            window.removeEventListener('resize', handleInput);
        };
    }, [suggestion]);

    // 初始渲染/更新后同步高度（保证首次加载高度正确）
    useEffect(() => {
        const timer = setTimeout(() => syncTextareaHeight(), 0);
        return () => clearTimeout(timer);
    }, [suggestion, textareaRef.current?.value]);

    return (
        <div className="relative w-full">
            <div
                ref={suggestionLayerRef}
                className={cn(
                    'pointer-events-none absolute top-0 left-0 w-full border border-transparent px-3 py-2 md:text-sm',
                    className
                )}
                aria-hidden="true"
            >
                <span className="inline-block text-transparent">{textareaRef.current?.value}</span>
                {suggestion.map((item, index) => (
                    <motion.span
                        className="inline-block text-gray-400"
                        initial={{ opacity: 0, y: 5, x: 5 }}
                        animate={{ opacity: 1, y: 0, x: 0 }}
                        transition={{ duration: 0.1 }}
                        onAnimationComplete={() => syncTextareaHeight()}
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
                    'placeholder:text-muted-foreground aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:bg-input/30 flex field-sizing-content max-h-64 min-h-16 w-full resize-none overflow-hidden rounded-md border border-transparent bg-transparent px-3 py-2 text-base shadow-xs shadow-transparent transition-[color,box-shadow] outline-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm',
                    className
                )}
                {...props}
            />
        </div>
    );
};

export default AutocompleteTextarea;
