import { TooltipProvider } from '@/components/ui/tooltip';
import './App.css';
import Chat from './components/chat/Chat';

function App() {
    return (
        <TooltipProvider>
            <div className="min-h-screen bg-slate-50">
                <div className="max-w-md mx-auto size-full">
                    <Chat />
                </div>
            </div>
        </TooltipProvider>
    );
}

export default App;
