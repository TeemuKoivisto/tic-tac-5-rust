const getEnv = (key: string, required = true): string => {
  const env = import.meta.env[key]
  if (!env && required) {
    throw new Error(`Environment variable ${key} was undefined!`)
  }
  return env
}

const parseInteger = (env?: string) => {
  try {
    return parseInt(env || '')
  } catch (err) {}
  return undefined
}

export const getPrefixedWS_URL = (url?: string) => {
  if (url && url.slice(0, 2) !== 'ws' && typeof window !== 'undefined') {
    return `ws://${window.location.host}${url.charAt(0) !== '/' ? '/' : ''}${url}`
  }
  return url
}

export const DEV = import.meta.env.DEV
export const API_URL = getEnv('VITE_API_URL')
export const WS_URL = getEnv('VITE_WS_URL')
