import { updatePreferences } from '@/actions/user.action';
import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarGroup,
    SidebarHeader,
    useSidebar,
} from '@/components/ui/sidebar';
import { produceAppState } from '@/lib/store';
import { useEffect } from 'react';
import { MyHeader } from './header/my';

export function AppSidebar() {
    const { open } = useSidebar();
    useEffect(() => {
        produceAppState(draft => {
            draft.pref.isSidebarOpen = open;
        });
        updatePreferences();
        console.info(`AppSidebar is changing: open=${open}`);
    }, [open]);
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
