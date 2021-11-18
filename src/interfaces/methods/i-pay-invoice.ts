export default interface IPayInvoice {
  /**
     * Payment request of the invoice to pay.
     */
  bolt11: string

  /**
     * Amount in sats if the invoice does not have one. If you specify an amount, you must specify
     * either amount or amountMsats, but not both.
     */
  amount?: number

  /**
     * Amount in msats if the invoice does not have one. If you specify an amount, you must specify
     * either amount or amountMsats, but not both.
     */
  amountMsats?: number
}
