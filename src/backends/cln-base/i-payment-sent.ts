export interface IPaymentSent {
  payment_preimage: string
  payment_hash: string
  created_at: number
  parts: number
  amount_msat: string
  amount_sent_msat: string
  status: string
  destination?: string
  warning_partial_completion?: string
}
