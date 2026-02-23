import { refreshCurrentUser } from '@/actions/user.action';
import App from '@/App';
import { getAppState } from '@/lib/store';
import { createBrowserRouter, redirect } from 'react-router';
import { ChatPage } from './page/chat-page';
import { Login } from './page/login';
import { Settings } from './page/settings';

export const router = createBrowserRouter([
    {
        path: '/',
        Component: App,
        children: [
            {
                index: true,
                loader: async () => {
                    if (!getAppState().logged_in) {
                        throw redirect('/login');
                    }
                    return null;
                },
                Component: ChatPage,
            },
            { path: 'settings', Component: Settings },
            {
                path: 'login',
                loader: async () => {
                    await refreshCurrentUser();
                    return redirect('/');
                },
                Component: Login,
            },
        ],
    },
]);
