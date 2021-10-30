export interface IInvoiceCreated {
  bolt11: string
  payment_hash: string
  payment_secret: string
  expires_at: number
}
