/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export type JsNode = Node
export class Node {
  constructor(backend: Backend, config: NodeConfig)
  createInvoice(invoice: CreateInvoiceParams): Promise<CreateInvoiceResult>
  getInfo(): Promise<NodeInfo>
}

export type Backend = "LndRest" | "LndGrpc" | "ClnGrpc" | "EclairRest" | "InvalidBackend";

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

export interface CreateInvoiceResult {
  label?: string | null;
  payment_hash: string;
  payment_request: string;
}

export type Network =
  | ("mainnet" | "testnet" | "regtest")
  | {
      unknown: string;
    };

export interface NodeConfig {
  macaroon?: string | null;
  password?: string | null;
  tls_certificate?: string | null;
  tls_client_certificate?: string | null;
  tls_client_key?: string | null;
  url?: string | null;
  username?: string | null;
}

export interface NodeInfo {
  backend: Backend;
  channels: ChannelStats;
  network: Network;
  node_pubkey: string;
  version: string;
}