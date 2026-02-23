export type UserState = {
    userId: string;
    userName: string;
    email: string;
};

export const INITIAL_USER_STATE: UserState = {
    userId: '',
    userName: '',
    email: '',
};
