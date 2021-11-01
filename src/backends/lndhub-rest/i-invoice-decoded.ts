export interface IInvoiceDecoded {
  route_hints: any[]
  features: { [key: string]: Feature }
  destination: string
  payment_hash: string
  num_satoshis: string
  timestamp: string
  expiry: string
  description: string
  description_hash: string
  fallback_addr: string
  cltv_expiry: string
  payment_addr: PaymentAddr
  num_msat: string
}

export interface Feature {
  name: string
  is_required: boolean
  is_known: boolean
}

export interface PaymentAddr {
  type: string
  data: number[]
}
