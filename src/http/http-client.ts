import * as https from 'https'
import * as http from 'http'

export const request = async (url: string, options: https.RequestOptions, data: unknown = null): Promise<any> => {
  let httpClient: typeof http | typeof https
  if (url.includes('https://')) {
    httpClient = https
  } else {
    httpClient = http
  }

  return await new Promise((resolve, reject) => {
    const req = httpClient.request(url, options, (res: any) => {
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
