import * as EventEmitter from 'events'
import { ICreateInvoice, Invoice } from '../interfaces'

export default interface IBackend {
  createInvoice: (invoice: ICreateInvoice) => Promise<Invoice>
  getInvoice: (hash: string) => Promise<Invoice>
  watchInvoices: () => EventEmitter
  startWatchingInvoices: () => void
}
