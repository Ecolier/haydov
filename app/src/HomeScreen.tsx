import React from 'react';
import {SafeAreaView} from 'react-native';
import {GET_USER} from './queries/get-user';
import {useQuery} from '@apollo/client';
import {Text} from 'react-native-paper';

function HomeScreen(): JSX.Element {
  const {data} = useQuery(GET_USER);
  return (
    <SafeAreaView>
      <Text>{data?.users[0]?.username}</Text>
    </SafeAreaView>
  );
}

export default HomeScreen;
