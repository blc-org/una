import * as https from 'https'
import { Backend, watchInvoices, URLToObject } from '..'
import { ILndHub, ICreateInvoice, IPayInvoice, IInvoice, IInvoicePaid } from '../../interfaces'
import { EHttpVerb, EInvoiceStatus } from '../../enums'
import { request } from '../../http'
import SocksProxyAgent from 'socks-proxy-agent'
import { IError, IInvoiceCreated, IInvoiceDecoded, ILoginAccess, IPaymentSent, IUserInvoice } from '.'

export default class LndHub extends Backend {
  protected readonly config: ILndHub
  private accessToken: string | undefined

  constructor (client: ILndHub, socksProxyUrl: string | null = null) {
    super()
    this.config = client
    this.setSocksProxyUrl(socksProxyUrl)

    if (this.config.uri != null) {
      const [credentialsToSplit, url] = this.config.uri.split('@')
      const credentials = credentialsToSplit.split('//')
      const [login, password] = credentials[1].split(':')
      this.config.url = url
      this.config.login = login
      this.config.password = password
    } else if (client.url !== null && client.login !== null && client.password !== null) {
      this.config.url = client.url
      this.config.login = client.login
      this.config.password = client.password
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

  public async payInvoice (invoice: IPayInvoice): Promise<IInvoicePaid> {
    const data = {
      invoice: invoice.bolt11
    }

    const body = this.prepareBody(data)
    const options = await this.getRequestOptions(EHttpVerb.POST, '/payinvoice')
    const response = await this.request(options, body) as IPaymentSent

    const result: IInvoicePaid = {
      paymentPreimage: Buffer.from(response.payment_preimage.data).toString('hex'),
      feesAmount: 0,
      feesAmountMsats: 0
    }

    return result
  }

  protected async getInvoices (): Promise<IUserInvoice[]> {
    const options = await this.getRequestOptions(EHttpVerb.GET, '/getuserinvoices')
    return await request(options) as IUserInvoice[]
  }

  public startWatchingInvoices (): void {
    // TODO: LndHub throw "Too many request" after only few requests. Find a better way to delay requests in watchInvoice()
    watchInvoices(this, 10000)
  }

  public async getPendingInvoices (): Promise<IInvoice[]> {
    const pendingInvoices = (await this.getInvoices()).filter(i => !this.isExpired(i.timestamp, i.expire_time) && i.ispaid === false)
    return pendingInvoices.map(invoice => this.toInvoice2(invoice))
  }

  private isExpired (invoiceTimestamp: number, expiry: number): boolean {
    return new Date().getTime() > (invoiceTimestamp + expiry) * 1000
  }

  protected toInvoice (invoice: IInvoiceDecoded, bolt11: string, isPaid: boolean): IInvoice {
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

  protected toInvoice2 (invoice: IUserInvoice): IInvoice {
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

  protected async getRequestOptions (method: EHttpVerb, path: string): Promise<https.RequestOptions> {
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
      ...URLToObject(this.config.url)
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
      ...URLToObject(this.config.url)
    }

    if (this.socksProxyUrl !== null) {
      const socks = new URL(this.socksProxyUrl)
      options.agent = new SocksProxyAgent.SocksProxyAgent({ hostname: socks.hostname, port: socks.port, protocol: socks.protocol })
    }

    const body = this.prepareBody({ login: this.config.login, password: this.config.password })
    const response = await request(options, body)

    if (response.error !== undefined) {
      const errorMessage = (response as IError).message
      throw new Error('LndHub login failed:' + errorMessage)
    }

    this.accessToken = (response as ILoginAccess).access_token
  }

  protected prepareBody (data: unknown): string | undefined {
    return data !== null ? JSON.stringify(data) : undefined
  }

  protected async request (options: https.RequestOptions, body: any = undefined): Promise<any> {
    return await request(options, body)
  }
}
