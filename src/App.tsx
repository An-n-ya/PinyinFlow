import { Outlet } from 'react-router';
import './App.css';
import { RootSideEffects } from './components/root/RootSideEffects';

function App() {
    return (
        <div>
            <RootSideEffects />
            <Outlet />
        </div>
    );
}

export default App;
