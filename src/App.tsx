import { Outlet, useNavigation } from 'react-router';
import './App.css';
import { SpinnerEmpty } from './app/page/spinner-empty';
import { RootSideEffects } from './components/root/RootSideEffects';

function App() {
    const navigation = useNavigation();
    const isNavigating = Boolean(navigation.location);
    return (
        <div>
            {isNavigating && <SpinnerEmpty />}
            <RootSideEffects />
            <Outlet />
        </div>
    );
}

export default App;
