export type PreferenceState = {
    userId: string;
    isSidebarOpen: boolean;
    enableCompleteInput: boolean;
    enableProofread: boolean;
};
export const INITIAL_PREFERENCE_STATE: PreferenceState = {
    userId: '',
    isSidebarOpen: false,
    enableCompleteInput: false,
    enableProofread: false,
};
