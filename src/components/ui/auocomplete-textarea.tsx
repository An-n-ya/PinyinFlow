import { motion } from 'motion/react';
import { useEffect, useRef } from 'react';

import { cn } from '@/lib/utils';

const AutocompleteTextarea = ({ className, suggestion, ...props }: AutocompleteTextareaProps) => {
    const textareaRef = useRef<HTMLTextAreaElement>(null);

    function autoResizeTextarea() {
        textareaRef.current?.setAttribute(
            'style',
            `height: ${textareaRef.current.scrollHeight}px;`
        );
    }

    useEffect(() => {
        autoResizeTextarea();
        textareaRef.current?.addEventListener('input', autoResizeTextarea);
        return () => {
            textareaRef.current?.removeEventListener('input', autoResizeTextarea);
        };
    });

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
                    'placeholder:text-muted-foreground aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive dark:bg-input/30 flex field-sizing-content min-h-16 w-full resize-none overflow-hidden rounded-md border border-transparent bg-transparent px-3 py-2 text-base shadow-xs shadow-transparent transition-[color,box-shadow] outline-none disabled:cursor-not-allowed disabled:opacity-50 md:text-sm',
                    className
                )}
                {...props}
            />
        </div>
    );
};

export default AutocompleteTextarea;
