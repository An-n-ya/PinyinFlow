import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarGroup,
    SidebarHeader,
    useSidebar,
} from '@/components/ui/sidebar';
import { getAppState } from '@/lib/store';
import { useEffect } from 'react';
import { MyHeader } from './header/my';

export function AppSidebar() {
    const { setOpen } = useSidebar();
    useEffect(() => {
        setOpen(getAppState().pref.isSidebarOpen);
    }, []);
    return (
        <Sidebar>
            <SidebarHeader>
                {' '}
                <MyHeader />
            </SidebarHeader>
            <SidebarContent>
                <SidebarGroup />
                <SidebarGroup />
            </SidebarContent>
            <SidebarFooter />
        </Sidebar>
    );
}
