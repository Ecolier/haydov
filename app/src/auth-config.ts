import {HAYDOV_AUTH_BASE_URL} from '@env';

const authConfig = {
  issuer: `${HAYDOV_AUTH_BASE_URL}/realms/master`,
  clientId: 'hasura',
  redirectUrl: 'haydov://home',
  scopes: ['openid', 'profile'],
};

export default authConfig;
