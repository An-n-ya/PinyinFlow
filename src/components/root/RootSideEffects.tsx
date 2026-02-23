import { refreshCurrentUser } from '@/actions/user.action';
import { useAsyncEffect } from '@/hooks/async.hooks';
import { getCurrentWebview } from '@tauri-apps/api/webview';
import { getAllWindows, getCurrentWindow } from '@tauri-apps/api/window';

export const RootSideEffects = () => {
    useAsyncEffect(async () => {
        await Promise.allSettled([refreshCurrentUser()]);
        const loaders: Promise<unknown>[] = [];
        await Promise.allSettled(loaders);
    }, []);

    let win = getCurrentWindow();
    if (win.label !== 'main') return null;
    getCurrentWebview().listen('settings-change', async () => {
        await refreshCurrentUser();
    });
    win.onCloseRequested(async e => {
        e.preventDefault();
        const allWindows = await getAllWindows();

        for (const win of allWindows) {
            if (win.label !== 'main') {
                await win.destroy();
            }
        }
        await win.destroy();
    });
    return null;
};
