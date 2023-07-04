import {AppRegistry} from 'react-native';
import {PaperProvider} from 'react-native-paper';
import HomeScreen from './src/HomeScreen';
import {name as appName} from './app.json';
import { prefetchConfiguration} from 'react-native-app-auth';
import {HAYDOV_API_BASE_URL} from '@env';
import {ApolloClient, InMemoryCache, ApolloProvider, createHttpLink} from '@apollo/client';
import { useEffect } from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import authLink from './src/auth-link';
import authConfig from './src/auth-config';
import AsyncStorage from '@react-native-async-storage/async-storage';

const httpLink = createHttpLink({
  uri: `${HAYDOV_API_BASE_URL}/v1/graphql`,
});

const client = new ApolloClient({
  uri: `${HAYDOV_API_BASE_URL}/v1/graphql`,
  cache: new InMemoryCache(),
  link: authLink.concat(httpLink),
});

const Stack = createNativeStackNavigator();

export default function Main() {
  
  useEffect(() => {
    prefetchConfiguration({
      warmAndPrefetchChrome: true,
      connectionTimeoutSeconds: 5,
      ...authConfig,
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
  