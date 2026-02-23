import { refreshCurrentUser } from '@/actions/user.action';
import App from '@/App';
import { getAppState } from '@/lib/store';
import { createBrowserRouter } from 'react-router';
import { ChatPage } from './page/chat-page';
import { Login } from './page/login';
import { Settings } from './page/settings';

export const router = createBrowserRouter([
    {
        path: '/',
        loader: async () => {
            if (!getAppState().logged_in) {
                await refreshCurrentUser();
            }
            return null;
        },
        Component: App,
        children: [
            {
                index: true,
                Component: ChatPage,
            },
            { path: 'settings', Component: Settings },
            {
                path: 'login',
                Component: Login,
            },
            // { path: 'about', Component: About },
            // {
            //     path: 'auth',
            //     Component: AuthLayout,
            //     children: [
            //         { path: 'login', Component: Login },
            //         { path: 'register', Component: Register },
            //     ],
            // },
            // {
            //     path: 'concerts',
            //     children: [
            //         { index: true, Component: ConcertsHome },
            //         { path: ':city', Component: ConcertsCity },
            //         { path: 'trending', Component: ConcertsTrending },
            //     ],
            // },
        ],
    },
]);
