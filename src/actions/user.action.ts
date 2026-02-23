import { getAppState, produceAppState } from '@/lib/store';
import { AppState } from '@/state/app.state';
import { PreferenceState } from '@/state/preference.state';
import { UserState } from '@/state/user.state';
import { invoke } from '@tauri-apps/api/core';

const DEV_USER_ID: string = '00000000-0000-0000-0000-000000000000';

function getUserId(_state: AppState): string {
    const mode = import.meta.env.VITE_MODE;
    if (mode === 'dev') {
        return DEV_USER_ID;
    }

    console.error('getUserId: unimplemented');

    return '';
}

export const refreshCurrentUser = async (): Promise<void> => {
    const state = getAppState();
    const userId = getUserId(state);
    const [user, preferences] = await Promise.all([
        invoke<UserState>('fetch_user_profiles', { userId }),
        invoke<PreferenceState>('fetch_user_preferences', { userId }),
    ]);
    produceAppState(draft => {
        draft.logged_in = true;
        draft.user = user;
        draft.pref = preferences;
    });
    console.info(`refersh current user: ${JSON.stringify(getAppState().user)}`);
    console.info(`refersh current pref: ${JSON.stringify(getAppState().pref)}`);
};

export const updatePreferences = async (): Promise<void> => {
    console.info('updaing user preferences');
    invoke('update_user_preferences', { pref: getAppState().pref });
};
