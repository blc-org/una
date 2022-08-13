/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export type JsNode = Node
export class Node {
  constructor(backend: Backend, config: NodeConfig)
  createInvoice(invoice: CreateInvoiceParams): string
  getInfo(): NodeInfo
}

export type Backend = "LndRest" | "LndGrpc" | "ClnRest" | "InvalidBackend";

export interface ChannelStats {
  active: number;
  inactive: number;
  pending: number;
}

export interface CreateInvoiceParams {
  amount?: number | null;
  amount_msat?: number | null;
  cltv_expiry?: number | null;
  description?: string | null;
  description_hash?: string | null;
  expire_in?: number | null;
  fallback_address?: string | null;
  label?: string | null;
  payment_preimage?: string | null;
}

export interface NodeConfig {
  certificate?: string | null;
  macaroon?: string | null;
  url?: string | null;
}

export interface NodeInfo {
  backend: Backend;
  channels: ChannelStats;
  node_pubkey: string;
  testnet: boolean;
  version: string;
}