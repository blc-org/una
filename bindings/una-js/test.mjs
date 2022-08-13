import { Node } from "./index.js";

const config = {
  url: "https://127.0.0.1:8084",
  macaroon: "a1b2c3",
  certificate: "a1b2c3",
};

const invoice = {
  amount: 2000,
  description: "test napi",
};

async function main() {
  const node = new Node("LndRest", config);
  const info = await node.getInfo();
  console.log(info);
  const payreq = await node.createInvoice(invoice);
  console.log(payreq);
}

main();
