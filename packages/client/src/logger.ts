import debug from 'debug'

export const logger = debug('game')

export const log = {
  print(type: 'debug' | 'info' | 'warn' | 'error', color: string, msg: string, obj?: any) {
    const str = type === 'info' ? msg : `%c ${type.toUpperCase()} -> ${msg}`
    if (obj && type === 'info') {
      logger(str, obj)
    } else if (obj) {
      logger(str, `color: ${color}`, obj)
    } else if (type === 'info') {
      logger(str)
    } else {
      logger(str, `color: ${color}`)
    }
  },
  debug(str: string, obj?: any) {
    return this.print('debug', '#c563ff', str, obj)
  },
  info(str: string, obj?: any) {
    return this.print('info', '', str, obj)
  },
  warn(str: string, obj?: any) {
    return this.print('warn', '#f3f32c', str, obj)
  },
  error(str: string, obj?: any) {
    return this.print('error', '#ff4242', str, obj)
  },
}

/**
 * Sets debug logging enabled/disabled.
 * @param enabled
 */
export const enableDebug = (enabled: boolean) => {
  if (enabled) {
    debug.enable('game')
  } else {
    debug.disable()
  }
}
