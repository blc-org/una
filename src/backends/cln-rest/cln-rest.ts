import * as https from 'https'
import { request } from '../../http'
import { ClnBase, URLToObject } from '..'
import { IClnRest } from '../../interfaces'
import { EHttpVerb } from '../../enums'
import SocksProxyAgent from 'socks-proxy-agent'

export default class ClnRest extends ClnBase {
  private readonly socksProxyUrl: string | null

  constructor (clnRest: IClnRest, socksProxyUrl: string | null = null) {
    super(clnRest)
    this.socksProxyUrl = socksProxyUrl
  }

  public async request (config: IClnRest, body: any): Promise<any> {
    const options: https.RequestOptions = {
      method: EHttpVerb.POST,
      path: '/v1/rpc',
      headers: {
        'Content-Type': 'application/json',
        macaroon: config.hexMacaroon,
        encodingtype: 'hex'
      },
      ...URLToObject(config.url)
    }

    if (this.socksProxyUrl !== null) {
      const socks = new URL(this.socksProxyUrl)
      options.agent = new SocksProxyAgent.SocksProxyAgent({ hostname: socks.hostname, port: socks.port, protocol: socks.protocol })
    }

    return await request(options, body)
  }
}
