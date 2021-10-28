import * as EventEmitter from 'events'
import { EclairRest, IBackend, LndRest } from './backends'
import { EBackendType } from './enums'
import { ICreateInvoice, IEclairRest, ILndRest, Invoice } from './interfaces'

export class Una {
  private readonly client: IBackend | undefined

  constructor (backend: EBackendType, connectionInformation: ConnectionInformation) {
    if (!this.verifyConnectionInformation(connectionInformation, backend)) {
      throw new Error('connectionInformation is not correct for the type ' + backend)
    }

    if (backend === EBackendType.LndRest) {
      const info = connectionInformation as ILndRest
      this.client = new LndRest({ url: info.url, hexMacaroon: info.hexMacaroon })
    } else if (backend === EBackendType.EclairRest) {
      const info = connectionInformation as IEclairRest
      this.client = new EclairRest({ url: info.url, user: info.user, password: info.password })
    } else {
      throw new Error('Backend not supported.')
    }

    this.client.startWatchingInvoices()
  }

  /**
       * Create an invoice
       * @param invoice {ICreateInvoice} object
       * @returns {Invoice} Invoice
       */
  public async createInvoice (invoice: ICreateInvoice): Promise<Invoice> {
    if (this.client === undefined) {
      throw new Error('No backend defined')
    }
    if (invoice.amount === undefined && invoice.amountMsats === undefined) {
      throw new Error('amount or amountMsat must be defined')
    }
    if (invoice.description !== undefined && invoice.descriptionHash !== undefined) {
      throw new Error('You must specify either description or descriptionHash, but not both')
    }

    return await this.client.createInvoice(invoice)
  }

  /**
       * Get an invoice previously created
       * @param hash hex encoded payment hash
       * @returns {Invoice} Invoice
       */
  public async getInvoice (hash: string): Promise<Invoice> {
    if (this.client === undefined) {
      throw new Error('No backend defined')
    }

    return await this.client.getInvoice(hash)
  }

  public watchInvoices (): EventEmitter {
    if (this.client === undefined) {
      throw new Error('No backend defined')
    }

    return this.client.watchInvoices()
  }

  private verifyConnectionInformation (connectionInformation: ConnectionInformation, backend: EBackendType): boolean {
    if (backend === EBackendType.LndRest) {
      const info = connectionInformation as ILndRest
      return info.url !== undefined && info.hexMacaroon !== undefined
    }
    if (backend === EBackendType.EclairRest) {
      const info = connectionInformation as IEclairRest
      return info.url !== undefined && info.user !== undefined && info.password !== undefined
    }

    return false
  }
}

type ConnectionInformation = ILndRest | IEclairRest
