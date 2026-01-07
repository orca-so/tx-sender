import {
  Address,
  address,
  createSolanaRpc,
  Rpc,
  SolanaRpcApi,
} from "@solana/kit";

/**
 * Creates an RPC client instance for interacting with the SVM blockchains using the provided RPC URL.
 *
 * @param {string} url - The RPC endpoint URL.
 * @returns {Rpc<SolanaRpcApi>} An RPC client configured with the Solana RPC API.
 *
 * @example
 * ```ts
 * const rpc = rpcFromUrl("https://api.mainnet-beta.solana.com");
 * const slot = await rpc.getSlot().send();
 * console.log(slot);
 * ```
 */
export function rpcFromUrl(url: string): Rpc<SolanaRpcApi> {
  return createSolanaRpc(url);
}

export function normalizeAddresses(
  addresses?: (string | Address)[],
): Address[] {
  return addresses?.map((addr) => address(addr)) ?? [];
}
