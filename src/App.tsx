import { TooltipProvider } from '@/components/ui/tooltip';
import './App.css';
import Chat from './components/chat/Chat';
import { AppSidebar } from './components/sidebar/app-sidebar';
import { Separator } from './components/ui/separator';
import { SidebarInset, SidebarProvider, SidebarTrigger } from './components/ui/sidebar';

function App() {
    return (
        <SidebarProvider>
            <TooltipProvider>
                <AppSidebar />
                <SidebarInset className="min-h-screen bg-slate-50">
                    <header className="bg-background flex h-16 flex-none shrink-0 items-center gap-2 border-b px-4">
                        <SidebarTrigger className="-ml-1" />
                        <Separator
                            orientation="vertical"
                            className="mr-2 data-[orientation=vertical]:h-4"
                        />
                        <h1>测试</h1>
                    </header>
                    <Chat />
                </SidebarInset>
            </TooltipProvider>
        </SidebarProvider>
    );
}

export default App;
