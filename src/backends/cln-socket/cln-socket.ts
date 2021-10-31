import * as net from 'net'
import { rpcRequest } from '../../rpc'
import { ClnBase } from '..'
import { IClnSocketUnix, IClnSocketTcp } from '../../interfaces'

export default class ClnSocket extends ClnBase {
  protected readonly clnConfig: IClnSocketUnix | IClnSocketTcp

  constructor (clnSocket: IClnSocketUnix | IClnSocketTcp) {
    super()
    this.clnConfig = clnSocket
  }

  protected async request (config: net.NetConnectOpts, body: any): Promise<any> {
    return await rpcRequest(config, body)
  }
}
