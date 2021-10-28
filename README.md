# UNA - Universal Node API

Una is a Lightning network node wrapper for LND, c-lightning, Eclair, LndHub, LNBits, more...

🚧 Una is still in development.

## Supported actions
 - [x] Create invoice
 - [x] Get invoice
 - [ ] Invoice events
 - More to come

## Supported backends
 - [x] LND (REST)
 - [ ] c-lightning
 - [x] Eclair (REST)
 - [ ] LndHub
 - [ ] LNBits
 - Want another implementation? [Open an issue](https://github.com/Dolu89/una/issues/new)

## How to use it
``` typescript
// ES Module
import { Una, EBackendType, ICreateInvoice, Invoice } from 'una-wrapper'
// Common JS
const { Una, EBackendType, ICreateInvoice, Invoice } = require('una-wrapper')

// LND Rest
const hexMacaroon = '0201036...311c811'
const unaWrapper = new Una(EBackendType.LndRest, { url: 'https://127.0.0.1:8080', hexMacaroon })

// Eclair Rest
const unaWrapper = new Una(EBackendType.EclairRest, { url: 'http://127.0.0.1:8080', user: '', password: 'eclairpw' })

/*
  Create an invoice of 15k satoshis with 'Hello' as memo
  Possible options are the following (from src/interface/i-create-invoice.ts)
  {
    amount: number
    amountMsats: number
    description: string
    descriptionHash: string
    expireIn?: number
    fallbackAddress?: string
    paymentPreimage?: string
  }
*/
const invoiceCreate: ICreateInvoice = { amount: 15000, description: 'Hello' }
const newInvoice = await unaWrapper.createInvoice(invoiceCreate)
// Get invoice created previously
const invoice = await unaWrapper.getInvoice(newInvoice.paymentHash)

/* newInvoice and invoice returns the same result
{
  bolt11: 'lnbcrt150u...0nwpszr675',
  amount: 15000,
  amountMsat: 15000000,
  creationDate: 2021-10-25T20:30:05.000Z,
  expiry: 3600,
  memo: 'Hello',
  settled: false,
  settleDate: null,
  paymentHash: '518a62665a...4ff4364f6f',
  preImage: '8a1ae80c77...fbfd1b1dd7'
}
*/

// Watch invoices changes
unaWrapper.watchInvoices().on('invoice-updated', (invoice: Invoice) => {
  console.log(invoice)
})
```
