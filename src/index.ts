import LndRest from './backends/lnd-rest'
import { EBackendType } from './enums/e-backend-type'
import Invoice from './interfaces/i-invoice'

export default class Una {
  private readonly backend: EBackendType
  private readonly lndRest: LndRest

  constructor (lndRestUrl: string, hexMacaroon: string) {
    this.backend = EBackendType.LndRest
    this.lndRest = new LndRest({ url: lndRestUrl, hexMacaroon })
  }

  /**
   * Create an invoice
   * @param amount amount in satoshis
   * @param memo memo
   * @returns Invoice
   */
  public async createInvoice (amount: number, memo: string): Promise<Invoice> {
    if (this.backend === EBackendType.LndRest) {
      return await this.lndRest.createInvoice(amount, memo)
    } else {
      throw new Error('No backend defined')
    }
  }

  /**
   * Get an invoice previously created
   * @param hash hex encoded payment hash
   * @returns Invoice
   */
  public async getInvoice (hash: string): Promise<Invoice> {
    if (this.backend === EBackendType.LndRest) {
      return await this.lndRest.getInvoice(hash)
    } else {
      throw new Error('No backend defined')
    }
  }
}
