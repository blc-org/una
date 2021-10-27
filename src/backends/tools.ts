export const base64ToHex = (base64: string): string => {
  return Buffer.from(base64, 'base64').toString('hex')
}

export const hexToBase64 = (base64: string): string => {
  return Buffer.from(base64, 'hex').toString('base64')
}
