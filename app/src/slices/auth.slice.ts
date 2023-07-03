import {PayloadAction, createSlice} from '@reduxjs/toolkit';

interface Authorization {
  accessToken: string;
  refreshToken: string;
}

const initialState: Partial<Authorization> = {};

const authSlice = createSlice({
  name: 'auth',
  initialState: initialState,
  reducers: {
    tokens(state, {payload}: PayloadAction<Authorization>) {
      state.accessToken = payload.accessToken;
      state.refreshToken = payload.refreshToken;
    },
  },
});

const {actions, reducer} = authSlice;
export const {tokens} = actions;
export default reducer;
