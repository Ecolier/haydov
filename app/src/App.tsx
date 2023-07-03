/**
 * Sample React Native App
 * https://github.com/facebook/react-native
 *
 * @format
 */

import React, {useCallback, useEffect} from 'react';
import {SafeAreaView, Text} from 'react-native';
import {useAppDispatch, useAppSelector} from './hooks';
import {Button} from 'react-native-paper';
import {authorize, prefetchConfiguration, refresh} from 'react-native-app-auth';
import {tokens} from './slices/auth.slice';
import {HAYDOV_AUTH_ISSUER_URL} from '@env';

const config = {
  issuer: `${HAYDOV_AUTH_ISSUER_URL}`,
  clientId: 'haydov-mobile',
  redirectUrl: 'haydov://home',
  scopes: ['openid', 'profile'],
};

function App(): JSX.Element {
  const user = useAppSelector(state => state.auth);
  const dispatch = useAppDispatch();
  useEffect(() => {
    prefetchConfiguration({
      warmAndPrefetchChrome: true,
      connectionTimeoutSeconds: 5,
      ...config,
    });
  }, []);
  const handleAuthorize = useCallback(async () => {
    try {
      const auth = await authorize(config);
      dispatch(
        tokens({
          accessToken: auth.accessToken,
          refreshToken: auth.refreshToken,
        }),
      );
    } catch (err: any) {
      console.log('Failed to log in', err.message);
    }
  }, [dispatch]);
  return (
    <SafeAreaView>
      {user && <Text>{JSON.stringify(user)}</Text>}
      <Button mode="contained" onPress={() => handleAuthorize()}>
        Sign In
      </Button>
    </SafeAreaView>
  );
}

export default App;
