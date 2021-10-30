import * as EventEmitter from 'events'
import { ClnSocket, ClnRest, EclairRest, IBackend, LndRest } from './backends'
import { EBackendType } from './enums'
import { IClnSocketUnix, IClnSocketTcp, IClnRest, ICreateInvoice, IEclairRest, ILndRest, Invoice } from './interfaces'

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
    } else if (backend === EBackendType.ClnSocketUnix) {
      const info = connectionInformation as IClnSocketUnix
      this.client = new ClnSocket({ path: info.path })
    } else if (backend === EBackendType.ClnSocketTcp) {
      const info = connectionInformation as IClnSocketTcp
      this.client = new ClnSocket({ host: info.host, port: info.port })
    } else if (backend === EBackendType.ClnRest) {
      const info = connectionInformation as IClnRest
      this.client = new ClnRest({ url: info.url, hexMacaroon: info.hexMacaroon })
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
    if ((invoice.description !== undefined && invoice.descriptionHash !== undefined) || (invoice.description === undefined && invoice.descriptionHash === undefined)) {
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
    if (backend === EBackendType.ClnSocketUnix) {
      const info = connectionInformation as IClnSocketUnix
      return info.path !== undefined
    }
    if (backend === EBackendType.ClnSocketTcp) {
      const info = connectionInformation as IClnSocketTcp
      return info.host !== undefined && info.port !== undefined
    }
    if (backend === EBackendType.ClnRest) {
      const info = connectionInformation as IClnRest
      return info.url !== undefined && info.hexMacaroon !== undefined
    }

    return false
  }
}

type ConnectionInformation = ILndRest | IEclairRest | IClnSocketUnix | IClnSocketTcp | IClnRest
