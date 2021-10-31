import { rpcRequest } from '../../rpc'
import { ClnBase } from '..'
import { IClnSocketUnix, IClnSocketTcp } from '../../interfaces'

export default class ClnSocket extends ClnBase {
  constructor (clnSocket: IClnSocketUnix | IClnSocketTcp) {
    super(clnSocket)
  }

  public async request (config: any, body: any): Promise<any> {
    return await rpcRequest(config, body)
  }
}
