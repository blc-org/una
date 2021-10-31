import * as https from 'https'
import { request } from '../../http'
import { IBackend, watchInvoices, URLToObject, cleanParams } from '..'
import { ICreateInvoice, IEclairRest, Invoice } from '../../interfaces'
import { EHttpVerb, EInvoiceStatus } from '../../enums'
import { IInvoiceCreated, IInvoiceLookup } from '.'
import { EventEmitter } from 'events'
import SocksProxyAgent from 'socks-proxy-agent'

export default class EclairRest implements IBackend {
  private readonly eclairRest: IEclairRest
  public readonly invoiceEmitter: EventEmitter
  public readonly invoicesToWatch: Invoice[]
  private readonly socksProxyUrl: string | null

  constructor (eclairRest: IEclairRest, socksProxyUrl: string | null = null) {
    this.eclairRest = eclairRest
    this.invoicesToWatch = []
    this.invoiceEmitter = new EventEmitter()
    this.socksProxyUrl = socksProxyUrl
  }

  public async createInvoice (invoice: ICreateInvoice): Promise<Invoice> {
    const amountMsat = invoice.amountMsats !== undefined ? invoice.amountMsats : invoice.amount * 1000

    const data: any = {
      amountMsat: amountMsat,
      description: invoice.description,
      descriptionHash: invoice.descriptionHash,
      expireIn: invoice.expireIn,
      fallbackAddress: invoice.fallbackAddress,
      paymentPreimage: invoice.paymentPreimage
    }

    const body = this.prepareBody(data)
    const options = this.getRequestOptions(EHttpVerb.POST, '/createinvoice')
    const response = await request(options, body) as IInvoiceCreated

    return await this.getInvoice(response.paymentHash)
  }

  public async getInvoice (hash: string): Promise<Invoice> {
    const data = {
      paymentHash: hash
    }

    const body = this.prepareBody(data)
    const options = this.getRequestOptions(EHttpVerb.POST, '/getreceivedinfo')
    const response = await request(options, body) as IInvoiceLookup

    return this.toInvoice(response)
  }

  public watchInvoices (): EventEmitter {
    return this.invoiceEmitter
  }

  public startWatchingInvoices (): void {
    watchInvoices(this)
  }

  public async getPendingInvoices (): Promise<Invoice[]> {
    const options = this.getRequestOptions(EHttpVerb.POST, '/listpendinginvoices')
    const initalInvoices = await request(options) as IInvoiceCreated[]
    return initalInvoices.map(i => this.toInvoice2(i))
  }

  private toDate (millisecond: string): Date {
    return new Date(Number(millisecond))
  }

  private toInvoice (invoice: IInvoiceLookup): Invoice {
    let status: EInvoiceStatus = EInvoiceStatus.Pending
    let settled = false
    if (invoice.status.type === 'pending') {
      status = EInvoiceStatus.Pending
    } else if (invoice.status.type === 'received') {
      status = EInvoiceStatus.Settled
      settled = invoice.status.type === 'received'
    } else if (invoice.status.type === 'expired') {
      status = EInvoiceStatus.Cancelled
    }

    return {
      bolt11: invoice.paymentRequest.serialized,
      amount: invoice.paymentRequest.amount / 1000,
      amountMsat: invoice.paymentRequest.amount,
      creationDate: this.toDate(invoice.paymentRequest.timestamp.toString()),
      expiry: Number(invoice.paymentRequest.expiry),
      memo: invoice.paymentRequest.description,
      settled,
      settleDate: settled ? this.toDate(invoice.status.receivedAt.toString()) : null,
      paymentHash: invoice.paymentRequest.paymentHash,
      preImage: invoice.paymentPreimage,
      status
    }
  }

  private toInvoice2 (invoice: IInvoiceCreated): Invoice {
    return {
      bolt11: invoice.serialized,
      amount: invoice.amount / 1000,
      amountMsat: invoice.amount,
      creationDate: this.toDate(invoice.timestamp.toString()),
      expiry: Number(invoice.expiry),
      memo: invoice.description,
      settled: false,
      settleDate: null,
      paymentHash: invoice.paymentHash,
      status: EInvoiceStatus.Pending
    }
  }

  private getRequestOptions (method: EHttpVerb, path: string): https.RequestOptions {
    const options: https.RequestOptions = {
      method,
      path,
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded',
        Authorization: 'Basic ' + Buffer.from(`${this.eclairRest.user}:${this.eclairRest.password}`).toString('base64')
      },
      ...URLToObject(this.eclairRest.url)
    }

    if (this.socksProxyUrl !== null) {
      const socks = new URL(this.socksProxyUrl)
      options.agent = new SocksProxyAgent.SocksProxyAgent({ hostname: socks.hostname, port: socks.port, protocol: socks.protocol, tls: { rejectUnauthorized: false } })
    }

    return options
  }

  private prepareBody (data: any): string {
    const cleanedParams = cleanParams(data)

    return new URLSearchParams(cleanedParams).toString()
  }
}
