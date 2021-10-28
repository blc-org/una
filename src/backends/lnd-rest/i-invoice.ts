export interface IInvoice {
  memo: string
  r_preimage: string
  r_hash: string
  value: string
  value_msat: string
  settled: boolean
  creation_date: string
  settle_date: string
  payment_request: string
  description_hash: null
  expiry: string
  fallback_addr: string
  cltv_expiry: string
  route_hints: any[]
  private: boolean
  add_index: string
  settle_index: string
  amt_paid: string
  amt_paid_sat: string
  amt_paid_msat: string
  state: string
  htlcs: any[]
  features: { [key: string]: IFeature }
  is_keysend: boolean
  payment_addr: string
}

interface IFeature {
  name: string
  is_required: boolean
  is_known: boolean
}
