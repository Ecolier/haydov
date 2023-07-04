import {authorize} from 'react-native-app-auth';
import authConfig from './auth-config';
import AsyncStorage from '@react-native-async-storage/async-storage';

export async function login() {
  const authorizeResult = await authorize(authConfig);
  AsyncStorage.multiSet([
    ['accessToken', authorizeResult.accessToken],
    ['refreshToken', authorizeResult.refreshToken],
  ]);
  return authorizeResult;
}
