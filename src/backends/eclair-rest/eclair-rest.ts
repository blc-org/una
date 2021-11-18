import * as https from 'https'
import { request } from '../../http'
import { Backend, URLToObject, cleanParams } from '..'
import { IEclairRest, ICreateInvoice, IPayInvoice, IInvoice, IInvoicePaid } from '../../interfaces'
import { EHttpVerb, EInvoiceStatus } from '../../enums'
import { IInvoiceCreated, IInvoiceLookup } from '.'
import SocksProxyAgent from 'socks-proxy-agent'

export default class EclairRest extends Backend {
  protected readonly config: IEclairRest

  constructor (eclairRest: IEclairRest, socksProxyUrl: string | null = null) {
    super()
    this.config = eclairRest
    this.setSocksProxyUrl(socksProxyUrl)
  }

  public async createInvoice (invoice: ICreateInvoice): Promise<IInvoice> {
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
    const response = await this.request(options, body) as IInvoiceCreated

    return await this.getInvoice(response.paymentHash)
  }

  public async getInvoice (hash: string): Promise<IInvoice> {
    const data = {
      paymentHash: hash
    }

    const body = this.prepareBody(data)
    const options = this.getRequestOptions(EHttpVerb.POST, '/getreceivedinfo')
    const response = await this.request(options, body) as IInvoiceLookup

    return this.toInvoice(response)
  }

  public async payInvoice (invoice: IPayInvoice): Promise<IInvoicePaid> {
    let amountMsat
    if (invoice.amount !== undefined) {
      amountMsat = invoice.amount * 1000
    } else if (invoice.amountMsats !== undefined) {
      amountMsat = invoice.amountMsats
    }

    const data = {
      invoice: invoice.bolt11,
      amountMsat: amountMsat
    }

    const body = this.prepareBody(data)
    const options = this.getRequestOptions(EHttpVerb.POST, '/payinvoice')
    const response = await this.request(options, body) as string

    const result: IInvoicePaid = {
      paymentPreimage: response
    }

    return result
  }

  public async getPendingInvoices (): Promise<IInvoice[]> {
    const options = this.getRequestOptions(EHttpVerb.POST, '/listpendinginvoices')
    const initalInvoices = await this.request(options) as IInvoiceCreated[]

    return initalInvoices.map(i => this.toInvoice2(i))
  }

  protected toInvoice (invoice: IInvoiceLookup): IInvoice {
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

  protected toInvoice2 (invoice: IInvoiceCreated): IInvoice {
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

  protected getRequestOptions (method: EHttpVerb, path: string): https.RequestOptions {
    const options: https.RequestOptions = {
      method,
      path,
      headers: {
        'Content-Type': 'application/x-www-form-urlencoded',
        Authorization: 'Basic ' + Buffer.from(`${this.config.user}:${this.config.password}`).toString('base64')
      },
      ...URLToObject(this.config.url)
    }

    if (this.socksProxyUrl !== null) {
      const socks = new URL(this.socksProxyUrl)
      options.agent = new SocksProxyAgent.SocksProxyAgent({ hostname: socks.hostname, port: socks.port, protocol: socks.protocol, tls: { rejectUnauthorized: false } })
    }

    return options
  }

  protected prepareBody (data: any): string {
    const cleanedParams = cleanParams(data)

    return new URLSearchParams(cleanedParams).toString()
  }

  protected async request (options: https.RequestOptions, body: any = undefined): Promise<any> {
    return await request(options, body)
  }
}
