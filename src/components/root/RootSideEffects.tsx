import { refreshCurrentUser } from '@/actions/user.action';
import { useAsyncEffect } from '@/hooks/async.hooks';
import { getCurrentWebview } from '@tauri-apps/api/webview';

export const RootSideEffects = () => {
    useAsyncEffect(async () => {
        await Promise.allSettled([refreshCurrentUser()]);
        const loaders: Promise<unknown>[] = [];
        await Promise.allSettled(loaders);
    }, []);

    getCurrentWebview().listen('settings-change', async () => {
        await refreshCurrentUser();
    });
    return null;
};
