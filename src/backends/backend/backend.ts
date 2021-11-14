import { EventEmitter } from 'events'
import { IBackend, watchInvoices } from '..'
import { ICreateInvoice, IInvoice } from '../../interfaces'

export default abstract class Backend implements IBackend {
  protected abstract readonly config: any
  protected socksProxyUrl: string | null

  public readonly invoiceEmitter: EventEmitter
  public readonly invoicesToWatch: IInvoice[]

  /**
   * Backend constructor
   */
  constructor () {
    this.invoicesToWatch = []
    this.invoiceEmitter = new EventEmitter()
    this.socksProxyUrl = null
  }

  protected setSocksProxyUrl (socksProxyUrl: string | null) {
    this.socksProxyUrl = socksProxyUrl
  }

  public watchInvoices (): EventEmitter {
    return this.invoiceEmitter
  }

  public startWatchingInvoices (): void {
    watchInvoices(this)
  }

  /**
   * Public methods to be overloaded
   */
  public abstract createInvoice (invoice: ICreateInvoice): Promise<IInvoice>
  public abstract getInvoice (hash: string): Promise<IInvoice>
  public abstract getPendingInvoices (): Promise<IInvoice[]>

  /**
   * Internal methods
   */
  protected abstract toInvoice (invoice: any): Promise<IInvoice> | IInvoice
  protected toDate (millisecond: number | string): Date {
    return new Date(Number(millisecond) * 1000)
  }

  /**
   * Internal network-related methods
   */
  protected abstract request (body: any): Promise<any>
  protected abstract request (options: any): Promise<any>
  protected abstract request (options: any, body: any): Promise<any>
  protected abstract prepareBody (data: any): string | undefined
  protected abstract prepareBody (method: string, params: any): string | undefined
}
