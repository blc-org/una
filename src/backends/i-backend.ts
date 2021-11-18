import * as EventEmitter from 'events'
import { ICreateInvoice, IPayInvoice, IInvoice, IInvoicePaid } from '../interfaces'

export default interface IBackend {
  invoiceEmitter: EventEmitter
  invoicesToWatch: IInvoice[]
  createInvoice: (invoice: ICreateInvoice) => Promise<IInvoice>
  getInvoice: (hash: string) => Promise<IInvoice>
  payInvoice: (invoice: IPayInvoice) => Promise<IInvoicePaid>
  watchInvoices: () => EventEmitter
  startWatchingInvoices: () => void
  getPendingInvoices: () => Promise<IInvoice[]>
}
