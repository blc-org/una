import fetch, { RequestInit } from 'node-fetch'
import { FormData } from 'formdata-node'
import { FormDataEncoder } from 'form-data-encoder'
import { Readable } from 'stream'
import { IBackend } from '..'
import { IEclairRest, Invoice } from '../../interfaces'
import { EHttpVerb } from '../../enums'
import { IInvoiceCreated, IInvoiceLookup } from '.'

export default class EclairRest implements IBackend {
  private readonly eclairRest: IEclairRest

  constructor (eclairRest: IEclairRest) {
    this.eclairRest = eclairRest
  }

  public async createInvoice (amount: number, memo: string): Promise<Invoice> {
    const form = new FormData()
    form.append('description', memo)
    form.append('amountMsat', String(amount * 1000))

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

  private toDate (millisecond: string): Date {
    return new Date(Number(millisecond))
  }

  private toInvoice (invoice: IInvoiceLookup): Invoice {
    const settled = invoice.status.type === 'received'
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
      preImage: invoice.paymentPreimage
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
