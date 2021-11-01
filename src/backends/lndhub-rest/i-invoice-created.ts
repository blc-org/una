export interface IInvoiceCreated {
  r_hash: RHash
  payment_request: string
  add_index: string
  pay_req: string
}

export interface RHash {
  type: string
  data: number[]
}
