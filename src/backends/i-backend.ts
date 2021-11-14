import * as EventEmitter from 'events'
import { ICreateInvoice, IInvoice } from '../interfaces'

export default interface IBackend {
  invoiceEmitter: EventEmitter
  invoicesToWatch: IInvoice[]
  createInvoice: (invoice: ICreateInvoice) => Promise<IInvoice>
  getInvoice: (hash: string) => Promise<IInvoice>
  watchInvoices: () => EventEmitter
  startWatchingInvoices: () => void
  getPendingInvoices: () => Promise<IInvoice[]>
}
