import { useAsyncEffect } from '@/hooks/async.hooks';

export const RootSideEffects = () => {
    useAsyncEffect(async () => {
        // await Promise.allSettled([refreshCurrentUser()]);
        const loaders: Promise<unknown>[] = [];
        await Promise.allSettled(loaders);
    }, []);
    return null;
};
