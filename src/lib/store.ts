import { AppState, INITIAL_APP_STATE } from '@/state/app.state';
import { produce } from 'immer';
import { create } from 'zustand';

export const appStore = create<AppState>(() => INITIAL_APP_STATE);

export const setAppState = appStore.setState;
export const getAppState = appStore.getState;
export const produceAppState = (fn: (draft: AppState) => void) => {
    setAppState(state => produce(state, fn));
};
