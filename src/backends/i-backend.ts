import { Invoice } from '../interfaces'

export default interface IBackend {
  createInvoice: (amount: number, memo: string) => Promise<Invoice>
  getInvoice: (hash: string) => Promise<Invoice>
}
