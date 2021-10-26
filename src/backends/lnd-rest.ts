import https from 'https'
import fetch, { RequestInit } from 'node-fetch'
import { EHttpVerb } from '../http/e-http-verb.js'
import Invoice from '../interfaces/i-invoice.js'
import ILndRest from '../interfaces/i-lnd-rest.js'
import IBackend from './i-backend.js'
import { base64ToHex } from './tools.js'

export default class LndRest implements IBackend {
  private readonly lndRest: ILndRest

  constructor (lndRest: ILndRest) {
    this.lndRest = lndRest
  }

  public async createInvoice (amount: number, memo: string): Promise<Invoice> {
    const body = {
      value_msat: amount * 1000,
      memo
    }
    const options = this.getRequestOptions(EHttpVerb.POST, body)
    const response = await fetch(this.lndRest.url + '/v1/invoices', options)
    const responseData = await response.json() as ILndInvoice
    return await this.getInvoice(base64ToHex(responseData.r_hash))
  }

  public async getInvoice (hash: string): Promise<Invoice> {
    const options = this.getRequestOptions(EHttpVerb.GET)
    const response = await fetch(this.lndRest.url + '/v1/invoice/' + hash, options)
    const responseData = await response.json() as ILndInvoice
    return this.toInvoice(responseData)
  }

  private toDate (millisecond: string): Date {
    return new Date(Number(millisecond) * 1000)
  }

  private toInvoice (invoice: ILndInvoice): Invoice {
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
      preImage: base64ToHex(invoice.r_preimage)
    }
  }

  private getRequestOptions (method: EHttpVerb, body: unknown = null): RequestInit {
    const agent = new https.Agent({
      rejectUnauthorized: false
    })
    return {
      method: method,
      agent,
      headers: {
        'Grpc-Metadata-macaroon': this.lndRest.hexMacaroon
      },
      body: body !== null ? JSON.stringify(body) : null
    }
  }
}

interface ILndInvoice {
  memo: string
  r_preimage: string
  r_hash: string
  value: string
  value_msat: string
  settled: boolean
  creation_date: string
  settle_date: string
  payment_request: string
  description_hash: null
  expiry: string
  fallback_addr: string
  cltv_expiry: string
  route_hints: any[]
  private: boolean
  add_index: string
  settle_index: string
  amt_paid: string
  amt_paid_sat: string
  amt_paid_msat: string
  state: string
  htlcs: any[]
  features: { [key: string]: IFeature }
  is_keysend: boolean
  payment_addr: string
}

interface IFeature {
  name: string
  is_required: boolean
  is_known: boolean
}
