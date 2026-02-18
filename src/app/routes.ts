import App from '@/App';
import { createBrowserRouter } from 'react-router';
import { ChatPage } from './page/chat-page';
import { Settings } from './page/settings';

export const router = createBrowserRouter([
    {
        path: '/',
        Component: App,
        children: [
            { index: true, Component: ChatPage },
            { path: 'settings', Component: Settings },
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
