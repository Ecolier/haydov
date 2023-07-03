import {AppRegistry} from 'react-native';
import {PaperProvider} from 'react-native-paper';
import HomeScreen from './src/HomeScreen';
import {name as appName} from './app.json';
import {Provider} from 'react-redux'
import { prefetchConfiguration, refresh} from 'react-native-app-auth';
import {HAYDOV_API_BASE_URL, HAYDOV_AUTH_BASE_URL} from '@env';
import {ApolloClient, InMemoryCache, ApolloProvider, createHttpLink, fromPromise} from '@apollo/client';
import {setContext} from '@apollo/client/link/context';
import AsyncStorage from '@react-native-async-storage/async-storage';
import { onError } from '@apollo/client/link/error';
import { useEffect } from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';

const config = {
  issuer: `${HAYDOV_AUTH_BASE_URL}/realms/master`,
  clientId: 'hasura',
  redirectUrl: 'haydov://home',
  scopes: ['openid', 'profile'],
};

const httpLink = createHttpLink({
  uri: `${HAYDOV_API_BASE_URL}/v1/graphql`,
});

const withAccessToken = setContext(async () => {
  const accessToken = await AsyncStorage.getItem('accessToken');
  return { accessToken };
});

const withHeaders = setContext((_, {accessToken}) => {
  return {
    headers: {
      Authorization: `Bearer ${accessToken}`
    }
  }
});

const errorLink = onError(({ graphQLErrors, operation, forward }) => {
  if (graphQLErrors) {
    for (let err of graphQLErrors) {
      switch (err.extensions.code) {
        case 'invalid-jwt':
        return fromPromise(AsyncStorage.getItem('refreshToken').then((refreshToken => refresh(config, {refreshToken}))))
        .flatMap((refreshResult) => {
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

const authFlowLink = errorLink.concat(withAccessToken).concat(withHeaders);

const client = new ApolloClient({
  uri: `${HAYDOV_API_BASE_URL}/v1/graphql`,
  cache: new InMemoryCache(),
  link: authFlowLink.concat(httpLink),
});

const Stack = createNativeStackNavigator();

export default function Main() {
  
  useEffect(() => {
    prefetchConfiguration({
      warmAndPrefetchChrome: true,
      connectionTimeoutSeconds: 5,
      ...config,
    });
  }, []);
  
  return (
    <ApolloProvider client={client}>
      <PaperProvider>
        <NavigationContainer>
          <Stack.Navigator>
            <Stack.Screen name="Home" component={HomeScreen} />
          </Stack.Navigator>
        </NavigationContainer>
      </PaperProvider>
    </ApolloProvider>
    );
  }
  
  AppRegistry.registerComponent(appName, () => Main);
  