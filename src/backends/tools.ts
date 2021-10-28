import { IBackend } from '.'
import { EInvoiceStatus } from '..'

export const base64ToHex = (base64: string): string => {
  return Buffer.from(base64, 'base64').toString('hex')
}

export const hexToBase64 = (base64: string): string => {
  return Buffer.from(base64, 'hex').toString('base64')
}

export const watchInvoices = (backend: IBackend): void => {
  setInterval(() => {
    backend.getPendingInvoices().then(pendingInvoices => {
      for (const pendingInvoice of pendingInvoices) {
        if (backend.invoicesToWatch.find(i => i.paymentHash === pendingInvoice.paymentHash) === undefined) {
          backend.invoicesToWatch.push(pendingInvoice)
        }
      }

      for (const invoiceToWatch of backend.invoicesToWatch) {
        backend.getInvoice(invoiceToWatch.paymentHash).then(invoice => {
          if (invoice.status !== invoiceToWatch.status) {
            backend.invoiceEmitter.emit('invoice-updated', invoice)
            if (invoice.status === EInvoiceStatus.Cancelled || invoice.status === EInvoiceStatus.Settled) {
              const indexToRemove = backend.invoicesToWatch.findIndex(i => i.paymentHash !== invoiceToWatch.paymentHash)
              backend.invoicesToWatch.splice(indexToRemove)
            }
          }
        }).catch(err => console.error('Unable to fetch invoice', err))
      }
    }).catch(err => console.error('Unable to fetch pending invoices', err))
  }, 5000)
}
