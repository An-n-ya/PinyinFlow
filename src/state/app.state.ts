import { INITIAL_PREFERENCE_STATE, PreferenceState } from './preference.state';
import { INITIAL_USER_STATE, UserState } from './user.state';

export type AppState = {
    logged_in: boolean;
    user: UserState;
    pref: PreferenceState;
};
export const INITIAL_APP_STATE: AppState = {
    logged_in: false,
    user: INITIAL_USER_STATE,
    pref: INITIAL_PREFERENCE_STATE,
};
