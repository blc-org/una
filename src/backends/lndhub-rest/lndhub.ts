import * as https from 'https'
import { IBackend, watchInvoices, URLToObject } from '..'
import { ICreateInvoice, ILndHub, IInvoice } from '../../interfaces'
import { EHttpVerb, EInvoiceStatus } from '../../enums'
import { EventEmitter } from 'events'
import { request } from '../../http'
import SocksProxyAgent from 'socks-proxy-agent'
import { IError, IInvoiceCreated, IInvoiceDecoded, ILoginAccess, IUserInvoice } from '.'

export default class LndHub implements IBackend {
  private readonly client: ILndHub
  public readonly invoiceEmitter: EventEmitter
  public readonly invoicesToWatch: IInvoice[]
  private readonly socksProxyUrl: string | null
  private accessToken: string | undefined

  constructor (client: ILndHub, socksProxyUrl: string | null = null) {
    this.client = client
    this.invoicesToWatch = []
    this.invoiceEmitter = new EventEmitter()
    this.socksProxyUrl = socksProxyUrl
    if (this.client.uri != null) {
      const [credentialsToSplit, url] = this.client.uri.split('@')
      const credentials = credentialsToSplit.split('//')
      const [login, password] = credentials[1].split(':')
      this.client.url = url
      this.client.login = login
      this.client.password = password
    } else if (client.url !== null && client.login !== null && client.password !== null) {
      this.client.url = client.url
      this.client.login = client.login
      this.client.password = client.password
    }
  }

  public async createInvoice (invoice: ICreateInvoice): Promise<IInvoice> {
    const amount = invoice.amount !== undefined ? invoice.amount : invoice.amountMsats * 1000

    const data = {
      amt: amount,
      memo: invoice.description
    }

    const body = this.prepareBody(data)
    const options = await this.getRequestOptions(EHttpVerb.POST, '/addinvoice')
    const response = await request(options, body) as IInvoiceCreated

    return await this.getInvoiceByBolt11(response.payment_request)
  }

  private async getInvoiceByBolt11 (bolt11: string): Promise<IInvoice> {
    const options = await this.getRequestOptions(EHttpVerb.GET, '/decodeinvoice?invoice=' + bolt11)
    const response = await request(options) as IInvoiceDecoded

    const optionsIsPaid = await this.getRequestOptions(EHttpVerb.GET, '/checkpayment/' + response.payment_hash)
    const { paid } = await request(optionsIsPaid) as { paid: boolean }
    return this.toInvoice(response, bolt11, paid)
  }

  public async getInvoice (hash: string): Promise<IInvoice> {
    const invoice = (await this.getInvoices()).find(i => i.payment_hash === hash)
    if (invoice == null) {
      throw new Error('Invoice not found')
    }

    return await this.getInvoiceByBolt11(invoice.payment_request)
  }

  private async getInvoices (): Promise<IUserInvoice[]> {
    const options = await this.getRequestOptions(EHttpVerb.GET, '/getuserinvoices')
    return await request(options) as IUserInvoice[]
  }

  public watchInvoices (): EventEmitter {
    return this.invoiceEmitter
  }

  public startWatchingInvoices (): void {
    // TODO: LndHub throw "Too many request" after only few requests. Find a beeter way to delay requests in watchInvoice()
    watchInvoices(this, 10000)
  }

  public async getPendingInvoices (): Promise<IInvoice[]> {
    const pendingInvoices = (await this.getInvoices()).filter(i => !this.isExpired(i.timestamp, i.expire_time) && i.ispaid === false)
    return pendingInvoices.map(invoice => this.toInvoice2(invoice))
  }

  private isExpired (invoiceTimestamp: number, expiry: number): boolean {
    return new Date().getTime() > (invoiceTimestamp + expiry) * 1000
  }

  private toDate (millisecond: string | number): Date {
    return new Date(Number(millisecond) * 1000)
  }

  private toInvoice (invoice: IInvoiceDecoded, bolt11: string, isPaid: boolean): IInvoice {
    let status: EInvoiceStatus = EInvoiceStatus.Pending
    let settled = false
    if (isPaid) {
      status = EInvoiceStatus.Settled
      settled = true
    } else if (this.isExpired(Number(invoice.timestamp), Number(invoice.expiry))) {
      status = EInvoiceStatus.Cancelled
    } else {
      status = EInvoiceStatus.Pending
    }

    return {
      bolt11: bolt11,
      amount: Number(invoice.num_satoshis),
      amountMsat: Number(invoice.num_msat),
      creationDate: this.toDate(invoice.timestamp),
      expiry: Number(invoice.expiry),
      memo: invoice.description,
      settled: settled,
      settleDate: null,
      paymentHash: invoice.payment_hash,
      preImage: null,
      status
    }
  }

  private toInvoice2 (invoice: IUserInvoice): IInvoice {
    let status: EInvoiceStatus = EInvoiceStatus.Pending
    let settled = false
    if (invoice.ispaid === true) {
      status = EInvoiceStatus.Settled
      settled = true
    } else if (!this.isExpired(Number(invoice.timestamp) * 1000, Number(invoice.expire_time))) {
      status = EInvoiceStatus.Pending
    } else {
      status = EInvoiceStatus.Cancelled
    }

    return {
      bolt11: invoice.payment_request,
      amount: invoice.amt,
      amountMsat: invoice.amt * 1000,
      creationDate: this.toDate(invoice.timestamp),
      expiry: invoice.expire_time,
      memo: invoice.description,
      settled: settled,
      settleDate: null,
      paymentHash: invoice.payment_hash,
      preImage: null,
      status
    }
  }

  private async getRequestOptions (method: EHttpVerb, path: string): Promise<https.RequestOptions> {
    if (this.accessToken === undefined) {
      await this.getAccessToken()
    }

    const options: https.RequestOptions = {
      method,
      path,
      headers: {
        'Content-type': 'application/json',
        Authorization: this.accessToken
      },
      ...URLToObject(this.client.url)
    }

    if (this.socksProxyUrl !== null) {
      const socks = new URL(this.socksProxyUrl)
      options.agent = new SocksProxyAgent.SocksProxyAgent({ hostname: socks.hostname, port: socks.port, protocol: socks.protocol })
    }

    return options
  }

  private async getAccessToken (): Promise<void> {
    const options: https.RequestOptions = {
      method: EHttpVerb.POST,
      path: '/auth?type=auth',
      headers: {
        'Content-type': 'application/json'
      },
      ...URLToObject(this.client.url)
    }

    if (this.socksProxyUrl !== null) {
      const socks = new URL(this.socksProxyUrl)
      options.agent = new SocksProxyAgent.SocksProxyAgent({ hostname: socks.hostname, port: socks.port, protocol: socks.protocol })
    }

    const body = this.prepareBody({ login: this.client.login, password: this.client.password })
    const response = await request(options, body)

    if (response.error !== undefined) {
      const errorMessage = (response as IError).message
      throw new Error('LndHub login failed:' + errorMessage)
    }

    this.accessToken = (response as ILoginAccess).access_token
  }

  private prepareBody (data: unknown): string | undefined {
    return data !== null ? JSON.stringify(data) : undefined
  }
}
