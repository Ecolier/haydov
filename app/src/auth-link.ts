import {fromPromise} from '@apollo/client';
import {setContext} from '@apollo/client/link/context';
import {onError} from '@apollo/client/link/error';
import AsyncStorage from '@react-native-async-storage/async-storage';
import {refresh} from 'react-native-app-auth';
import authConfig from './auth-config';
import {login} from './auth';

const withAccessToken = setContext(async () => {
  const accessToken = await AsyncStorage.getItem('accessToken');
  return {accessToken};
});

const withHeaders = setContext((_, {accessToken}) => {
  return {
    headers: {
      Authorization: `Bearer ${accessToken}`,
    },
  };
});

const errorLink = onError(({graphQLErrors, operation, forward}) => {
  if (graphQLErrors) {
    for (let err of graphQLErrors) {
      switch (err.extensions.code) {
        case 'invalid-jwt':
          return fromPromise(
            AsyncStorage.getItem('refreshToken').then(refreshToken => {
              if (!refreshToken) {
                return login();
              }
              return refresh(authConfig, {refreshToken}).catch(() => login());
            }),
          ).flatMap(refreshResult => {
            AsyncStorage.multiSet([
              ['accessToken', refreshResult.accessToken],
              ['refreshToken', refreshResult.refreshToken],
            ]);
            return forward(operation);
          });
      }
    }
  }
});

const authLink = errorLink.concat(withAccessToken).concat(withHeaders);

export default authLink;
