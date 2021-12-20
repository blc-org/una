import { randomUUID } from 'crypto'
import { IBackend } from '.'
import { EInvoiceStatus } from '..'

export const base64ToHex = (base64: string): string => {
  return Buffer.from(base64, 'base64').toString('hex')
}

export const hexToBase64 = (base64: string): string => {
  return Buffer.from(base64, 'hex').toString('base64')
}

export const watchInvoices = (backend: IBackend, intervalMs: number | null = null): void => {
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
            if (invoice.status !== EInvoiceStatus.Pending) {
              const indexToRemove = backend.invoicesToWatch.findIndex(i => i.paymentHash !== invoiceToWatch.paymentHash)
              backend.invoicesToWatch.splice(indexToRemove)
            }
          }
        }).catch(err => console.error('Unable to fetch invoice', err))
      }
    }).catch(err => console.error('Unable to fetch pending invoices', err))
  }, intervalMs ?? 5000)
}

export const URLToObject = (urlStr: string): { protocol: string, hostname: string, port: string, pathname: string } => {
  const url = new URL(urlStr)

  return {
    protocol: url.protocol,
    hostname: url.hostname,
    port: url.port,
    pathname: url.pathname
  }
}

export const generateUUID = (): string => randomUUID()

export const cleanParams = (params: any): any => {
  const cleanedParams: any = {}
  for (const key in params) {
    const currentValue = params[key]
    if (currentValue !== undefined) {
      cleanedParams[key] = currentValue
    }
  }

  return cleanedParams
}
