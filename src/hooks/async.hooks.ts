import { DependencyList, useEffect, useRef } from 'react';

export const useAsyncEffect = (
    effect: () => Promise<(() => void) | void>,
    deps: DependencyList
): void => {
    const cleanupRef = useRef<(() => void) | void>(undefined);
    const runningRef = useRef<Promise<void> | null>(null);

    useEffect(() => {
        let cancelled = false;

        const run = async () => {
            // Wait for any previous effect to complete before running cleanup
            if (runningRef.current) {
                await runningRef.current;
            }

            // Run the previous cleanup if it exists
            if (cleanupRef.current) {
                cleanupRef.current();
                cleanupRef.current = undefined;
            }

            if (cancelled) return;

            // Run the new effect and store its cleanup
            const cleanup = await effect();
            if (!cancelled) {
                cleanupRef.current = cleanup;
            } else if (cleanup) {
                // If cancelled while effect was running, clean up immediately
                cleanup();
            }
        };

        runningRef.current = run();

        return () => {
            cancelled = true;
            // Schedule cleanup to run after current effect completes
            runningRef.current?.then(() => {
                if (cleanupRef.current) {
                    cleanupRef.current();
                    cleanupRef.current = undefined;
                }
            });
        };
    }, deps);
};
