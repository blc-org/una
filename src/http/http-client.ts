import * as http from 'http'
import * as https from 'https'
import { EHttpVerb } from '../enums'

export const request = async (options: https.RequestOptions, data: unknown = null): Promise<any> => {
  const httpClient = options.protocol === 'https:' ? https : http

  return await new Promise((resolve, reject) => {
    const req = httpClient.request(options, (res: any) => {
      res.setEncoding('utf8')

      let responseBody = ''
      res.on('data', (chunk: string) => {
        responseBody += chunk
      })

      res.on('end', () => {
        const parsedBody: { error?: string } = JSON.parse(responseBody)

        if (parsedBody.error != null) {
          return reject(parsedBody.error)
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
