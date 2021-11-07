import { rpcRequest } from '../../rpc'
import { ClnBase } from '..'
import { IClnSocketUnix, IClnSocketTcp } from '../../interfaces'

export default class ClnSocket extends ClnBase {
  protected readonly config: IClnSocketUnix | IClnSocketTcp

  constructor (clnSocket: IClnSocketUnix | IClnSocketTcp) {
    super()
    this.config = clnSocket
  }

  protected async request (body: any): Promise<any> {
    return await rpcRequest(this.config, body)
  }
}
