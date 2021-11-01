export interface IUserInvoice {
  r_hash: RHash
  payment_request: string
  add_index: string
  pay_req: string
  description: string
  payment_hash: string
  amt: number
  expire_time: number
  timestamp: number
  type: string
  ispaid?: boolean
}

export interface RHash {
  type: string
  data: number[]
}
