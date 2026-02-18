import { Label } from '@/components/ui/label';
import { RadioGroup, RadioGroupItem } from '@/components/ui/radio-group';
import { Separator } from '@/components/ui/separator';
import {
    Sidebar,
    SidebarContent,
    SidebarGroup,
    SidebarGroupContent,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarProvider,
} from '@/components/ui/sidebar';
import { Switch } from '@/components/ui/switch';

import type { IconName } from 'lucide-react/dynamic';
import { DynamicIcon } from 'lucide-react/dynamic';
import React, { useState } from 'react';
type Switcher = {
    type: 'switcher';
    name: string;
    action?: (checked: boolean) => void;
};
type DropDown = {
    type: 'dropdown';
    name: string;
    sub: settingItem[];
};
type Radio = {
    type: 'radio';
    options: string[];
    default: string;
    action?: () => void;
};
type settingItem = Switcher | DropDown | Radio;
type navItem = {
    name: string;
    icon: IconName;
    sub?: navItem[];
    settings?: settingItem[];
};

const nav: navItem[] = [
    {
        name: '通用',
        icon: 'menu',
        sub: [
            {
                name: '外观',
                icon: 'paintbrush',
                settings: [
                    {
                        type: 'switcher',
                        name: '启用暗黑模式',
                        action: checked => {
                            console.log(checked);
                        },
                    },
                    {
                        type: 'radio',
                        options: ['跟随系统', '浅色', '深色'],
                        default: '跟随系统',
                    },
                ],
            },
            { name: '消息', icon: 'message-circle' },
            { name: '语言和地区', icon: 'globe' },
        ],
    },
    { name: '快捷键', icon: 'keyboard' },
    { name: '设备', icon: 'video' },
    { name: '通知', icon: 'bell' },
    { name: '账户', icon: 'link' },
    { name: '隐私', icon: 'lock' },
];
interface SetttingItemProps {
    item: settingItem;
    key: string;
}
function SettingItem({ item, key }: SetttingItemProps) {
    let node;
    switch (item.type) {
        case 'radio':
            node = (
                <RadioGroup
                    defaultValue={item.default}
                    className="flex w-fit flex-row justify-between gap-8"
                >
                    {item.options.map((option, i) => (
                        <div key={`${option}-${i}`} className="flex items-center gap-3">
                            <RadioGroupItem value={option} id={option} />
                            <Label htmlFor={option}>{option}</Label>
                        </div>
                    ))}
                </RadioGroup>
            );
            break;
        case 'switcher':
            node = (
                <div className="flex items-center justify-between gap-3">
                    <Label htmlFor={item.name}>{item.name}</Label>
                    <Switch id={item.name} onCheckedChange={item.action} />
                </div>
            );
            break;
        case 'dropdown':
            node = <div>Dropdown</div>;
    }
    return (
        <React.Fragment key={key}>
            <div className="py-4">{node}</div>
            <Separator />
        </React.Fragment>
    );
}

export function Settings() {
    const [active, setActive] = useState<navItem>(nav[0]);
    const handleSidebarMenuClick = (e: React.MouseEvent<HTMLButtonElement>, menu_item: navItem) => {
        e.preventDefault();
        setActive(menu_item);
    };
    return (
        <SidebarProvider className="items-start">
            <Sidebar collapsible="none" className="flex h-screen w-40">
                <SidebarContent>
                    <SidebarGroup>
                        <SidebarGroupContent>
                            <SidebarMenu>
                                {nav.map(item => (
                                    <SidebarMenuItem key={item.name}>
                                        <SidebarMenuButton
                                            asChild
                                            onClick={e => handleSidebarMenuClick(e, item)}
                                            isActive={item.name === active.name}
                                        >
                                            <a href="#">
                                                <DynamicIcon name={item.icon} />
                                                <span>{item.name}</span>
                                            </a>
                                        </SidebarMenuButton>
                                    </SidebarMenuItem>
                                ))}
                            </SidebarMenu>
                        </SidebarGroupContent>
                    </SidebarGroup>
                </SidebarContent>
            </Sidebar>
            <main className="flex h-screen flex-1 flex-col overflow-hidden">
                <header className="flex h-16 shrink-0 items-center gap-2 transition-[width,height] ease-linear group-has-data-[collapsible=icon]/sidebar-wrapper:h-12">
                    <div className="flex items-center gap-2 px-4">
                        <h1>{active.name}</h1>
                    </div>
                </header>
                <div className="flex flex-1 flex-col gap-4 overflow-y-auto p-4 pt-0">
                    {active.sub?.map((item, i) => (
                        <>
                            <h1>{item.name}</h1>
                            <div
                                key={i}
                                className="bg-muted/50 aspect-video max-w-3xl rounded-xl px-4"
                            >
                                {item.settings?.map((item, i) => (
                                    <SettingItem item={item} key={`${item.type}-${i}`} />
                                ))}{' '}
                            </div>
                        </>
                    ))}
                </div>
            </main>
        </SidebarProvider>
    );
}
