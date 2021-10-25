import * as https from 'https'

export const request = async (url: string, options: https.RequestOptions, data: unknown = null): Promise<any> => {
  return await new Promise((resolve, reject) => {
    const req = https.request(url, options, (res: any) => {
      res.setEncoding('utf8')
      let responseBody = ''

      res.on('data', (chunk: string) => {
        responseBody += chunk
      })

      res.on('end', () => {
        resolve(JSON.parse(responseBody))
      })
    })

    req.on('error', (err: unknown) => {
      reject(err)
    })

    if (options.method !== 'GET') {
      req.write(data)
    }
    req.end()
  })
}
