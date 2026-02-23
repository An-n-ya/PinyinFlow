import { useEffect } from 'react';
import { redirect } from 'react-router';

export function Login() {
    useEffect(() => {
        console.info('redirecting to home page');
        redirect('/');
    }, []);
    return (
        <div>
            <h1>Login</h1>
        </div>
    );
}
