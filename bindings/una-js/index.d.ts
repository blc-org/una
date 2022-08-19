/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export type JsNode = Node
export class Node {
  constructor(backend: Backend, config: NodeConfig)
  createInvoice(invoice: CreateInvoiceParams): Promise<CreateInvoiceResult>
  getInfo(): Promise<NodeInfo>
  payInvoice(invoice: PayInvoiceParams): Promise<PayInvoiceResult>
  getInvoice(payementHash: String): Promise<Invoice>
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

export type InvoiceStatus = "Pending" | "Settled" | "Cancelled" | "Accepted";

export interface Invoice {
  amount: number;
  amount_msat: number;
  bolt11: string;
  creation_date: number;
  expiry: number;
  memo: string;
  payment_hash: string;
  pre_image?: string | null;
  settle_date?: number | null;
  settled: boolean;
  status: InvoiceStatus;
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

export interface PayInvoiceParams {
  amount?: number | null;
  amount_msat?: number | null;
  max_fee_msat?: number | null;
  max_fee_percent?: number | null;
  max_fee_sat?: number | null;
  payment_request: string;
}

export interface PayInvoiceResult {
  fees_msat?: number | null;
  payment_hash: string;
  payment_preimage: string;
}