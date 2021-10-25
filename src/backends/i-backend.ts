import Invoice from '../interfaces/i-invoice'

export default interface IBackend {
  createInvoice: (amount: number, memo: string) => Promise<Invoice>
  getInvoice: (hash: string) => Promise<Invoice>
}
