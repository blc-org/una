export interface IListInvoices {
  invoices: [ IListedInvoice ]
}

export interface IListedInvoice {
  label: string
  description: string
  payment_hash: string
  status: string
  expires_at: number
  msatoshi: number
  amount_msat?: string
  bolt11?: string
  bolt12?: string
  local_offer_id?: string
  payer_note?: string
  /* If status is "paid" */
  pay_index?: number
  amount_received_msat?: number
  paid_at?: number
  payment_preimage?: string
}
