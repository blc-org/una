import * as https from 'https'
import { request } from '../http/http-client'
import Invoice from '../interfaces/i-invoice'
import ILndRest from '../interfaces/i-lnd-rest'
import IBackend from './i-backend'
import { base64ToHex } from './tools'

export default class LndRest implements IBackend {
  private readonly lndRest: ILndRest

  constructor (lndRest: ILndRest) {
    this.lndRest = lndRest
  }

  public async createInvoice (amount: number, memo: string): Promise<Invoice> {
    const data = JSON.stringify({
      value_msat: amount * 1000,
      memo
    })

    const options: https.RequestOptions = {
      method: 'POST',
      rejectUnauthorized: false,
      headers: {
        'Grpc-Metadata-macaroon': this.lndRest.hexMacaroon
      }
    }
    const invoiceCreated = await request(this.lndRest.url + '/v1/invoices', options, data)
    return await this.getInvoice(base64ToHex(invoiceCreated.r_hash))
  }

  public async getInvoice (hash: string): Promise<Invoice> {
    const options: https.RequestOptions = {
      method: 'GET',
      rejectUnauthorized: false,
      headers: {
        'Grpc-Metadata-macaroon': this.lndRest.hexMacaroon
      }
    }
    const invoice: LndInvoice = await request(this.lndRest.url + '/v1/invoice/' + hash, options)
    return this.toInvoice(invoice)
  }

  private toDate (millisecond: string): Date {
    return new Date(Number(millisecond) * 1000)
  }

  private toInvoice (invoice: LndInvoice): Invoice {
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
}

interface LndInvoice {
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
  features: { [key: string]: Feature }
  is_keysend: boolean
  payment_addr: string
}

interface Feature {
  name: string
  is_required: boolean
  is_known: boolean
}
