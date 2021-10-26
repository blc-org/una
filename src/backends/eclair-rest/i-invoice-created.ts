export interface IInvoiceCreated {
  prefix: string
  timestamp: number
  nodeId: string
  serialized: string
  description: string
  paymentHash: string
  expiry: number
  minFinalCltvExpiry: number
  amount: number
  features: Features
}

interface Features {
  activated: Activated
  unknown: any[]
}

interface Activated {
  var_onion_optin: string
  payment_secret: string
  basic_mpp: string
}
