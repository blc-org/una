export interface IPaymentSent {
  payment_error: string
  payment_preimage: string
  route: any[]
  payment_hash: string
  payment_route: PaymentRoute
}

interface PaymentRoute {
  total_fees: number
  total_fees_msat: number
}
