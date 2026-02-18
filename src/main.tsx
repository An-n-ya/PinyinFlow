import ReactDOM from 'react-dom/client';
import { RouterProvider } from 'react-router/dom';
import { router } from './app/routes';

import '@fontsource/roboto/300.css';
import '@fontsource/roboto/400.css';
import '@fontsource/roboto/500.css';
import '@fontsource/roboto/700.css';

import { debug, error, info, warn } from '@tauri-apps/plugin-log';

function forwardConsole(
    fnName: 'log' | 'debug' | 'info' | 'warn' | 'error',
    logger: (message: string) => Promise<void>
) {
    const original = console[fnName];
    console[fnName] = message => {
        original(message);
        logger(message);
    };
}

forwardConsole('debug', debug);
forwardConsole('info', info);
forwardConsole('warn', warn);
forwardConsole('error', error);

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <RouterProvider router={router} />
);
