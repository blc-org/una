import * as https from 'https'
import { request } from '../../http'
import { ClnBase, URLToObject } from '..'
import { IClnRest } from '../../interfaces'
import { EHttpVerb } from '../../enums'

export default class ClnRest extends ClnBase {
  protected readonly clnConfig: IClnRest

  constructor (clnRest: IClnRest) {
    super()
    this.clnConfig = clnRest
  }

  protected async request (config: IClnRest, body: any): Promise<any> {
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

    return await request(options, body)
  }
}
