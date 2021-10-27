export default interface ICreateInvoice {
  /**
     * Amount in msats. You must specify either amount or amountMsats, but not both.
     */
  amount: number

  /**
     * Amount in sats. You must specify either amount or amountMsats, but not both.
     */
  amountMsats: number

  /**
     * Description. You must specify either description or descriptionHash, but not both.
     */
  description: string

  /**
     * Description hash in 32 bytes hex string. You must specify either description or descriptionHash, but not both.
     */
  descriptionHash: string

  /**
     * Number of seconds that the invoice will be valid
     */
  expireIn?: number

  /**
     *An on-chain fallback address to receive the payment
    */
  fallbackAddress?: string

  /**
     * A user defined input for the generation of the paymentHash
     */
  paymentPreimage?: string
}
