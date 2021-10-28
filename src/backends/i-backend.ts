import * as EventEmitter from 'events'
import { ICreateInvoice, Invoice } from '../interfaces'

export default interface IBackend {
  invoiceEmitter: EventEmitter
  invoicesToWatch: Invoice[]
  createInvoice: (invoice: ICreateInvoice) => Promise<Invoice>
  getInvoice: (hash: string) => Promise<Invoice>
  watchInvoices: () => EventEmitter
  startWatchingInvoices: () => void
  getPendingInvoices: () => Promise<Invoice[]>
}
