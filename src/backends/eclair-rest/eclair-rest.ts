import fetch, { RequestInit } from 'node-fetch'
import { FormData } from 'formdata-node'
import { FormDataEncoder } from 'form-data-encoder'
import { Readable } from 'stream'
import { IBackend } from '..'
import { ICreateInvoice, IEclairRest, Invoice } from '../../interfaces'
import { EHttpVerb, EInvoiceStatus } from '../../enums'
import { IInvoiceCreated, IInvoiceLookup } from '.'
import { EventEmitter } from 'events'

export default class EclairRest implements IBackend {
  private readonly eclairRest: IEclairRest
  private readonly invoiceEmitter: EventEmitter
  private readonly invoicesToWatch: Invoice[]

  constructor (eclairRest: IEclairRest) {
    this.eclairRest = eclairRest
    this.invoicesToWatch = []
    this.invoiceEmitter = new EventEmitter()
  }

  public async createInvoice (invoice: ICreateInvoice): Promise<Invoice> {
    const amountMsat = invoice.amountMsats !== undefined ? invoice.amountMsats : invoice.amount * 1000

    const form = new FormData()
    form.append('amountMsat', String(amountMsat))

    if (invoice.description !== undefined && invoice.descriptionHash === undefined) {
      form.append('description', invoice.description)
    } else if (invoice.descriptionHash !== undefined && invoice.description === undefined) {
      form.append('descriptionHash', invoice.descriptionHash)
    } else {
      throw new Error('You must specify either description or descriptionHash, but not both')
    }
    if (invoice.expireIn !== undefined) {
      form.append('expireIn', invoice.expireIn)
    }
    if (invoice.fallbackAddress !== undefined) {
      form.append('fallbackAddress', invoice.fallbackAddress)
    }
    if (invoice.paymentPreimage !== undefined) {
      form.append('paymentPreimage', invoice.paymentPreimage)
    }

    const options = this.getRequestOptions(EHttpVerb.POST, form)
    const response = await fetch(`${this.eclairRest.url}/createinvoice`, options)
    const responseData = await response.json() as IInvoiceCreated

    return await this.getInvoice(responseData.paymentHash)
  }

  public async getInvoice (hash: string): Promise<Invoice> {
    const data = new FormData()
    data.append('paymentHash', hash)
    const options = this.getRequestOptions(EHttpVerb.POST, data)
    const response = await fetch(this.eclairRest.url + '/getreceivedinfo', options)
    const responseData = await response.json() as IInvoiceLookup

    return this.toInvoice(responseData)
  }

  public watchInvoices (): EventEmitter {
    return this.invoiceEmitter
  }

  public startWatchingInvoices (): void {
    setInterval(() => {
      this.getPendingInvoices().then(pendingInvoices => {
        for (const pendingInvoice of pendingInvoices) {
          if (this.invoicesToWatch.find(i => i.paymentHash === pendingInvoice.paymentHash) === undefined) {
            this.invoicesToWatch.push(pendingInvoice)
          }
        }

        for (const invoiceToWatch of this.invoicesToWatch) {
          this.getInvoice(invoiceToWatch.paymentHash).then(invoice => {
            if (invoice.status !== invoiceToWatch.status) {
              this.invoiceEmitter.emit('invoice-updated', invoice)
              if (invoice.status === EInvoiceStatus.Cancelled || invoice.status === EInvoiceStatus.Settled) {
                const indexToRemove = this.invoicesToWatch.findIndex(i => i.paymentHash !== invoiceToWatch.paymentHash)
                this.invoicesToWatch.splice(indexToRemove)
              }
            }
          }).catch(err => console.error('Unable to fetch invoice', err))
        }
      }).catch(err => console.error('Unable to fetch pending invoices', err))
    }, 5000)
  }

  private async getPendingInvoices (): Promise<Invoice[]> {
    const options = this.getRequestOptions(EHttpVerb.POST, new FormData())
    const results = await fetch(this.eclairRest.url + '/listpendinginvoices', options)
    const initalInvoices = await results.json() as IInvoiceCreated[]
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

  private getRequestOptions (method: EHttpVerb, form: FormData): RequestInit {
    const encoder = new FormDataEncoder(form)

    return {
      method: method,
      headers: {
        ...encoder.headers,
        Authorization: 'Basic ' + Buffer.from(`${this.eclairRest.user}:${this.eclairRest.password}`).toString('base64')
      },
      body: Readable.from(encoder)
    }
  }
}
