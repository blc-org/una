import { IBackend, watchInvoices, generateUUID, cleanParams } from '..'
import { ICreateInvoice, Invoice, IClnSocketUnix, IClnSocketTcp, IClnRest } from '../../interfaces'
import { EInvoiceStatus } from '../../enums'
import { IInvoiceDecode, IInvoiceCreated, IListInvoices, IListedInvoice } from '.'
import { EventEmitter } from 'events'

export default class ClnBase implements IBackend {
  private readonly clnConfig: IClnSocketUnix | IClnSocketTcp | IClnRest
  public readonly invoiceEmitter: EventEmitter
  public readonly invoicesToWatch: Invoice[]

  constructor (clnConfig: IClnSocketUnix | IClnSocketTcp | IClnRest) {
    this.clnConfig = clnConfig
    this.invoicesToWatch = []
    this.invoiceEmitter = new EventEmitter()
  }

  public async createInvoice (invoice: ICreateInvoice): Promise<Invoice> {
    const amountMsat = invoice.amountMsats !== undefined ? invoice.amountMsats : invoice.amount * 1000
    const label = invoice.label !== undefined ? invoice.label : generateUUID()

    const data: any = {
      msatoshi: amountMsat,
      label: label,
      description: invoice.description,
      expiry: invoice.expireIn,
      preimage: invoice.paymentPreimage
    }

    if (invoice.fallbackAddress !== undefined) {
      data.fallbacks = [invoice.fallbackAddress]
    }

    const body = this.prepareBody('invoice', data)
    const response = await this.request(this.clnConfig, body) as IInvoiceCreated

    return await this.getInvoice(response.payment_hash)
  }

  public async getInvoice (hash: string): Promise<Invoice> {
    const result = await this.listInvoices(hash)

    return await this.toInvoice(result.invoices[0])
  }

  private async listInvoices (hash?: string): Promise<IListInvoices> {
    const data = {
      payment_hash: hash
    }

    const body = this.prepareBody('listinvoices', data)
    const response = await this.request(this.clnConfig, body) as IListInvoices

    return response
  }

  private async decodeInvoice (bolt11: string): Promise<IInvoiceDecode> {
    const data = {
      bolt11
    }

    const body = this.prepareBody('decodepay', data)
    const response = await this.request(this.clnConfig, body) as IInvoiceDecode

    return response
  }

  public watchInvoices (): EventEmitter {
    return this.invoiceEmitter
  }

  public startWatchingInvoices (): void {
    watchInvoices(this)
  }

  public async getPendingInvoices (): Promise<Invoice[]> {
    const body = this.prepareBody('listinvoices')
    const allInvoices = await this.request(this.clnConfig, body) as IListInvoices
    const pendingInvoices = allInvoices.invoices.filter(i => i.status === 'unpaid')
    const finalInvoices = await pendingInvoices.map(async i => await this.toInvoice(i))

    return await Promise.all(finalInvoices)
  }

  private toDate (millisecond: number | string): Date {
    return new Date(Number(millisecond) * 1000)
  }

  private async toInvoice (invoice: IListedInvoice): Promise<Invoice> {
    if (invoice.bolt11 !== null) {
      throw new Error('Invoice is not a bolt11')
    }

    const decodedInvoice = await this.decodeInvoice(invoice.bolt11)

    let status: EInvoiceStatus = EInvoiceStatus.Pending
    let settled = false
    if (invoice.status === 'unpaid') {
      status = EInvoiceStatus.Pending
    } else if (invoice.status === 'paid') {
      status = EInvoiceStatus.Settled
      settled = invoice.status === 'paid'
    } else if (invoice.status === 'expired') {
      status = EInvoiceStatus.Cancelled
    }

    return {
      bolt11: invoice.bolt11,
      amount: invoice.msatoshi / 1000,
      amountMsat: Number(invoice.amount_msat?.replace('msat', '')),
      creationDate: this.toDate(decodedInvoice.created_at),
      expiry: Number(decodedInvoice.expiry),
      memo: invoice.description !== undefined ? invoice.description : '',
      settled,
      settleDate: settled && invoice.paid_at !== undefined ? this.toDate(invoice.paid_at) : null,
      paymentHash: invoice.payment_hash,
      preImage: invoice.payment_preimage !== undefined ? invoice.payment_preimage : null,
      status
    }
  }

  public prepareBody (method: string, params: any = {}): string | undefined {
    const cleanedParams = cleanParams(params)

    const body = {
      jsonrpc: '2.0',
      method,
      cleanedParams,
      id: 0
    }

    return JSON.stringify(body)
  }

  public async request (config: any, body: any): Promise<any> { }
}
