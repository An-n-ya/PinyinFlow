export type PreferenceState = {
    userId: string;
    isSidebarOpen: boolean;
};
export const INITIAL_PREFERENCE_STATE: PreferenceState = {
    userId: '',
    isSidebarOpen: false,
};
