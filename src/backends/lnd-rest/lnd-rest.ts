import * as https from 'https'
import { Backend, URLToObject, base64ToHex } from '..'
import { ICreateInvoice, ILndRest, IInvoice } from '../../interfaces'
import { EHttpVerb, EInvoiceStatus } from '../../enums'
import { ILndInvoice } from '.'
import { request } from '../../http'
import SocksProxyAgent from 'socks-proxy-agent'

export default class LndRest extends Backend {
  protected readonly config: ILndRest

  constructor (lndRest: ILndRest, socksProxyUrl: string | null = null) {
    super()
    this.config = lndRest
    this.setSocksProxyUrl(socksProxyUrl)
  }

  public async createInvoice (invoice: ICreateInvoice): Promise<IInvoice> {
    const amountMsat = invoice.amountMsats !== undefined ? invoice.amountMsats : invoice.amount * 1000

    const data = {
      value_msat: amountMsat,
      expiry: invoice.expireIn,
      fallback_addr: invoice.fallbackAddress,
      paymentPreimage: invoice.paymentPreimage,
      memo: invoice.description,
      description_hash: Buffer.from(invoice.descriptionHash, 'hex').toString('base64')
    }

    const body = this.prepareBody(data)
    const options = this.getRequestOptions(EHttpVerb.POST, '/v1/invoices')
    const response = await this.request(options, body) as ILndInvoice

    return await this.getInvoice(base64ToHex(response.r_hash))
  }

  public async getInvoice (hash: string): Promise<IInvoice> {
    const options = this.getRequestOptions(EHttpVerb.GET, '/v1/invoice/' + hash)
    const response = await this.request(options) as ILndInvoice

    return this.toInvoice(response)
  }

  public async getPendingInvoices (): Promise<IInvoice[]> {
    const options = this.getRequestOptions(EHttpVerb.GET, '/v1/invoices?pending_only=true')
    const initalInvoices = await this.request(options) as { invoices: ILndInvoice[] }

    return initalInvoices.invoices.map(i => this.toInvoice(i))
  }

  protected toInvoice (invoice: ILndInvoice): IInvoice {
    let status: EInvoiceStatus = EInvoiceStatus.Pending
    if (invoice.state === 'OPEN') {
      status = EInvoiceStatus.Pending
    } else if (invoice.state === 'SETTLED') {
      status = EInvoiceStatus.Settled
    } else if (invoice.state === 'CANCELED') {
      status = EInvoiceStatus.Cancelled
    } else if (invoice.state === 'ACCEPTED') {
      status = EInvoiceStatus.Accepted
    }

    return {
      bolt11: invoice.payment_request,
      amount: Number(invoice.value),
      amountMsat: Number(invoice.value_msat),
      creationDate: this.toDate(invoice.creation_date),
      expiry: Number(invoice.expiry),
      memo: invoice.memo,
      settled: invoice.settled,
      settleDate: invoice.settle_date === '0' ? null : this.toDate(invoice.settle_date),
      paymentHash: base64ToHex(invoice.r_hash),
      preImage: base64ToHex(invoice.r_preimage),
      status
    }
  }

  protected getRequestOptions (method: EHttpVerb, path: string): https.RequestOptions {
    let agent: https.Agent

    if (this.socksProxyUrl !== null) {
      const socks = new URL(this.socksProxyUrl)
      agent = new SocksProxyAgent.SocksProxyAgent({ hostname: socks.hostname, port: socks.port, protocol: socks.protocol, tls: { rejectUnauthorized: false } })
    } else {
      agent = new https.Agent({
        rejectUnauthorized: false
      })
    }

    return {
      method,
      path,
      agent,
      headers: {
        'Grpc-Metadata-macaroon': this.config.hexMacaroon
      },
      ...URLToObject(this.config.url)
    }
  }

  protected prepareBody (data: unknown): string | undefined {
    return data !== null ? JSON.stringify(data) : undefined
  }

  protected async request (options: https.RequestOptions, body: any = undefined): Promise<any> {
    return await request(options, body)
  }
}
