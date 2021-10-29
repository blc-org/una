import * as http from 'http'
import * as https from 'https'
import { EHttpVerb } from '../enums'

export const request = async (url: string, options: https.RequestOptions, data: unknown = null): Promise<any> => {
  const httpClient = url.includes('https://') ? https : http

  return await new Promise((resolve, reject) => {
    const req = httpClient.request(url, options, (res: any) => {
      res.setEncoding('utf8')

      let responseBody = ''
      res.on('data', (chunk: string) => {
        responseBody += chunk
      })

      res.on('end', () => {
        const parsedBody: { error?: string } = JSON.parse(responseBody)

        if (parsedBody.error != null) {
          return reject(new Error(parsedBody.error))
        }

        resolve(parsedBody)
      })
    })

    req.on('error', (err: unknown) => {
      reject(err)
    })

    if (options.method !== EHttpVerb.GET && data !== null) {
      req.write(data)
    }

    req.end()
  })
}
