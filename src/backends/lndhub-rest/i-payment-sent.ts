export interface IPaymentSent {
  payment_error: string
  payment_preimage: IPreimage
  route: any
}

interface IPreimage {
  type: string
  data: Buffer
}
