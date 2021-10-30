export interface IInvoiceDecode {
  currency: string
  created_at: number
  expiry: number
  payee: string
  payment_hash: string
  signature: string
  min_final_cltv_expiry: number
  msatoshi: number
  amount_msat?: string
  description?: string
  description_hash?: string
  payment_secret?: string
  features?: string
  fallbacks?: [ Fallback ]
  routes?: [ [ RouteHop ]]
  extra?: [ Extra ]
}

interface Fallback {
  type: string
  hex: string
  addr?: string
}

interface RouteHop {
  pubkey: string
  short_channel_id: string
  fee_base_msat: number
  fee_proportional_millionths: number
  cltv_expiry_delta: number
}

interface Extra {
  tag: string
  data: string
}
