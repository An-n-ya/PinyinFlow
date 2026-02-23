import Chat from '@/components/chat/Chat';
import { AppSidebar } from '@/components/sidebar/app-sidebar';
import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import { SidebarInset, SidebarProvider, SidebarTrigger } from '@/components/ui/sidebar';
import { TooltipProvider } from '@/components/ui/tooltip';
import { getAppState } from '@/lib/store';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { X } from 'lucide-react';

export function ChatPage() {
    const handleCloseWindow = async () => {
        await getCurrentWebviewWindow()?.close();
    };
    return (
        <SidebarProvider defaultOpen={getAppState().pref.isSidebarOpen}>
            <TooltipProvider>
                <AppSidebar />
                <SidebarInset className="min-h-screen bg-slate-50">
                    <header className="bg-background flex h-16 flex-none shrink-0 items-center justify-between border-b px-4">
                        <div className="flex items-center gap-2">
                            <SidebarTrigger className="-ml-1" />
                            <Separator
                                orientation="vertical"
                                className="mr-2 data-[orientation=vertical]:h-4"
                            />
                            <h1>测试</h1>
                        </div>
                        <Button
                            variant="ghost"
                            size="icon"
                            className="rounded-full"
                            onClick={handleCloseWindow}
                        >
                            <X />
                        </Button>
                    </header>
                    <Chat />
                </SidebarInset>
            </TooltipProvider>
        </SidebarProvider>
    );
}
