import { AudioLines } from 'lucide-react';
import type { IconName } from 'lucide-react/dynamic';

import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { SidebarMenu, SidebarMenuButton, SidebarMenuItem } from '@/components/ui/sidebar';
import { DynamicIcon } from 'lucide-react/dynamic';

type MenuItem = {
    title: string;
    href: string;
    icon_name: IconName;
};

const menu_items: MenuItem[] = [
    {
        title: '设置',
        href: '/docs/overview',
        icon_name: 'settings',
    },
    {
        title: '帮助',
        href: '/guides/overview',
        icon_name: 'badge-question-mark',
    },
    {
        title: '关于',
        href: '/components/accordion',
        icon_name: 'info',
    },
];

import { Webview } from '@tauri-apps/api/webview';
import { Window } from '@tauri-apps/api/window';

function new_window(window_id: string) {
    const appWindow = new Window(window_id);

    console.log('creating window in new_window...');
    appWindow.once('tauri://created', async function () {
        // `new Webview` Should be called after the window is successfully created,
        // or webview may not be attached to the window since window is not created yet.
        console.log('creating window...');
        const webview = new Webview(appWindow, window_id, {
            url: '/settings',

            // create a webview with specific logical position and size
            x: 0,
            y: 0,
            width: 800,
            height: 600,
        });

        webview.once('tauri://created', function () {
            console.log('webview created!');
            // webview successfully created
        });
        webview.once('tauri://error', function (e) {
            console.log(e);
            console.log('webview creating failed!');
            // an error happened creating the webview
        });

        // emit an event to the backend
        await webview.emit('some-event', 'data');
        // listen to an event from the backend
        const unlisten = await webview.listen('event-name', _e => {});
        unlisten();
    });
}

export function MyHeader() {
    const handleSelect = (item: MenuItem) => {
        if (item.title === '设置') {
            console.log('creating settings window...');
            new_window('settings');
        }
        console.log('select:', item);
    };
    return (
        <SidebarMenu>
            <SidebarMenuItem>
                <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                        <SidebarMenuButton
                            size="lg"
                            className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
                        >
                            <div className="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg">
                                <AudioLines className="size-4" />
                            </div>
                            <div className="flex flex-col gap-0.5 leading-none">
                                <span className="font-medium">VoiceRelay</span>
                                <span className="">v0.0.1</span>
                            </div>
                        </SidebarMenuButton>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent
                        className="w-(--radix-dropdown-menu-trigger-width)"
                        align="start"
                    >
                        {menu_items.map(item => (
                            <DropdownMenuItem key={item.title} onSelect={() => handleSelect(item)}>
                                <DynamicIcon name={item.icon_name} />
                                {item.title}{' '}
                            </DropdownMenuItem>
                        ))}
                    </DropdownMenuContent>
                </DropdownMenu>
            </SidebarMenuItem>
        </SidebarMenu>
    );
}
