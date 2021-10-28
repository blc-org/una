import { EInvoiceStatus } from '..'

export default interface Invoice {
  bolt11: string
  memo: string
  amount: number
  amountMsat: number
  preImage?: string
  paymentHash: string
  settled: boolean
  settleDate: Date | null
  creationDate: Date
  expiry: number
  status: EInvoiceStatus
}
