import * as net from 'net'

export const rpcRequest = async (options: net.NetConnectOpts, data: any = null): Promise<any> => {
  return await new Promise((resolve, reject) => {
    const client = net.createConnection(options)
    client.setEncoding('utf-8')

    client.on('connect', () => {
      client.write(data)
    })

    let responseBody = ''
    client.on('data', (chunk: string) => {
      responseBody += chunk
      if (responseBody.slice(-3).toString() === '}\n\n') {
        try {
          const responseObj = JSON.parse(responseBody.toString())
          client.end()
          if (responseObj.error !== undefined) {
            reject(responseObj.error)
          } else {
            resolve(responseObj.result)
          }
        } catch (err) {
          reject(err)
        }
      }
    })
  })
}
