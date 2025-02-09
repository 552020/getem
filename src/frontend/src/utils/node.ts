import {
  getApplicationIdFromLocalStorage,
  getNodeUrlFromLocalStorage,
  getContextIdFromLocalStorage,
  setContextIdToLocalStorage,
  getJWTObject,
  setApplicationIdToLocalStorage,
} from './storage';

export const getNodeUrl = (): string => {
  // Get from environment variables
  const nodeUrl = getNodeUrlFromLocalStorage();

  if (!nodeUrl) {
    const nodeUrlFromEnv = import.meta.env.VITE_NODE_URL;
    console.warn(
      'Node URL not found in localStorage, using environment variable',
    );
    return nodeUrlFromEnv;
  }

  return nodeUrl;
};

export function getContextId(): string {
  const storageContextId = getContextIdFromLocalStorage();

  if (!storageContextId) {
    const jwtToken = getJWTObject();
    const envKey: string = jwtToken?.context_id ?? '';
    setContextIdToLocalStorage(envKey);
    return envKey;
  }

  return storageContextId ?? '';
}

export const getApplicationId = (): string | null => {
  const applicationId = getApplicationIdFromLocalStorage();
  if (!applicationId) {
    const applicationIdFromEnv = import.meta.env.VITE_APPLICATION_ID;
    setApplicationIdToLocalStorage(applicationIdFromEnv);
    return applicationIdFromEnv;
  }
  return applicationId;
};

export function getNearEnvironment(): string {
  return import.meta.env['VITE_NEAR_ENVIRONMENT'] ?? 'testnet';
}

// Name has been changed between getApplicationId and getStorageApplicationId
// export function getStorageApplicationId(): string {
//   const storageApplicationId = getApplicationIdFromLocalStorage();

//   if (!storageApplicationId) {
//     const envKey: string = import.meta.env['VITE_APPLICATION_ID'] ?? '';
//     setApplicationIdToLocalStorage(envKey);
//     return envKey;
//   }

//   return storageApplicationId ?? '';
// }
