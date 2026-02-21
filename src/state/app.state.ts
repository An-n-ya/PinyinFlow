import { INITIAL_PREFERENCE_STATE, PreferenceState } from './preference.state';

export type AppState = {
    pref: PreferenceState;
};
export const INITIAL_APP_STATE: AppState = {
    pref: INITIAL_PREFERENCE_STATE,
};
