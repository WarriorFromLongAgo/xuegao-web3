import type * as types from './types';
import type { ConfigOptions, FetchResponse } from 'api/dist/core'
import Oas from 'oas';
import APICore from 'api/dist/core';
import definition from './openapi.json';

class SDK {
  spec: Oas;
  core: APICore;

  constructor() {
    this.spec = Oas.init(definition);
    this.core = new APICore(this.spec, 'ava-cloud/Beta (api/6.1.1)');
  }

  /**
   * Optionally configure various options that the SDK allows.
   *
   * @param config Object of supported SDK options and toggles.
   * @param config.timeout Override the default `fetch` request timeout of 30 seconds. This number
   * should be represented in milliseconds.
   */
  config(config: ConfigOptions) {
    this.core.setConfig(config);
  }

  /**
   * If the API you're using requires authentication you can supply the required credentials
   * through this method and the library will magically determine how they should be used
   * within your API request.
   *
   * With the exception of OpenID and MutualTLS, it supports all forms of authentication
   * supported by the OpenAPI specification.
   *
   * @example <caption>HTTP Basic auth</caption>
   * sdk.auth('username', 'password');
   *
   * @example <caption>Bearer tokens (HTTP or OAuth 2)</caption>
   * sdk.auth('myBearerToken');
   *
   * @example <caption>API Keys</caption>
   * sdk.auth('myApiKey');
   *
   * @see {@link https://spec.openapis.org/oas/v3.0.3#fixed-fields-22}
   * @see {@link https://spec.openapis.org/oas/v3.1.0#fixed-fields-22}
   * @param values Your auth credentials for the API; can specify up to two strings or numbers.
   */
  auth(...values: string[] | number[]) {
    this.core.setAuth(...values);
    return this;
  }

  /**
   * If the API you're using offers alternate server URLs, and server variables, you can tell
   * the SDK which one to use with this method. To use it you can supply either one of the
   * server URLs that are contained within the OpenAPI definition (along with any server
   * variables), or you can pass it a fully qualified URL to use (that may or may not exist
   * within the OpenAPI definition).
   *
   * @example <caption>Server URL with server variables</caption>
   * sdk.server('https://{region}.api.example.com/{basePath}', {
   *   name: 'eu',
   *   basePath: 'v14',
   * });
   *
   * @example <caption>Fully qualified server URL</caption>
   * sdk.server('https://eu.api.example.com/v14');
   *
   * @param url Server URL
   * @param variables An object of variables to replace into the server URL.
   */
  server(url: string, variables = {}) {
    this.core.setServer(url, variables);
  }

  /**
   * Get the health of the service
   *
   * @throws FetchError<503, types.HealthCheckResponse503> The Health Check is not successful
   */
  healthCheck(): Promise<FetchResponse<200, types.HealthCheckResponse200>> {
    return this.core.fetch('/v1/health-check', 'get');
  }

  /**
   * Triggers reindexing of token metadata for an NFT token. Reindexing can only be called
   * once per hour for each NFT token.
   *
   * @summary Reindex NFT metadata
   */
  reindexNft(metadata: types.ReindexNftMetadataParam): Promise<FetchResponse<number, unknown>> {
    return this.core.fetch('/v1/chains/{chainId}/nfts/collections/{address}/tokens/{tokenId}:reindex', 'post', metadata);
  }

  /**
   * Lists tokens for an NFT contract.
   *
   * @summary List tokens
   */
  listTokens(metadata: types.ListTokensMetadataParam): Promise<FetchResponse<200, types.ListTokensResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/nfts/collections/{address}/tokens', 'get', metadata);
  }

  /**
   * Gets token details for a specific token of an NFT contract.
   *
   * @summary Get token details
   */
  getTokenDetails(metadata: types.GetTokenDetailsMetadataParam): Promise<FetchResponse<200, types.GetTokenDetailsResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/nfts/collections/{address}/tokens/{tokenId}', 'get', metadata);
  }

  /**
   * Gets operation details for the given operation id.
   *
   * @summary Get operation
   */
  getOperationResult(metadata: types.GetOperationResultMetadataParam): Promise<FetchResponse<200, types.GetOperationResultResponse200>> {
    return this.core.fetch('/v1/operations/{operationId}', 'get', metadata);
  }

  /**
   * Trigger a transaction export operation with given parameters.
   *
   * The transaction export operation runs asynchronously in the background. The status of
   * the job can be retrieved from the `/v1/operations/:operationId` endpoint using the
   * `operationId` returned from this endpoint.
   *
   * @summary Create transaction export operation
   */
  postTransactionExportJob(body: types.PostTransactionExportJobBodyParam): Promise<FetchResponse<200, types.PostTransactionExportJobResponse200>> {
    return this.core.fetch('/v1/operations/transactions:export', 'post', body);
  }

  /**
   * Gets the details of a single transaction on one of the Primary Network chains.
   *
   * @summary Get transaction
   */
  getTxByHash(metadata: types.GetTxByHashMetadataParam): Promise<FetchResponse<200, types.GetTxByHashResponse200>> {
    return this.core.fetch('/v1/networks/{network}/blockchains/{blockchainId}/transactions/{txHash}', 'get', metadata);
  }

  /**
   * Lists the latest transactions on one of the Primary Network chains.
   *
   * Transactions are filterable by addresses, txTypes, and timestamps. When querying for
   * latest transactions without an address parameter, filtering by txTypes and timestamps is
   * not supported. An address filter must be provided to utilize txTypes and timestamp
   * filters.
   *
   * Given that each transaction may return a large number of UTXO objects, bounded only by
   * the maximum transaction size, the query may return less transactions than the provided
   * page size. The result will contain less results than the page size if the number of
   * utxos contained in the resulting transactions reach a performance threshold.
   *
   * @summary List latest transactions
   */
  listLatestPrimaryNetworkTransactions(metadata: types.ListLatestPrimaryNetworkTransactionsMetadataParam): Promise<FetchResponse<200, types.ListLatestPrimaryNetworkTransactionsResponse200>> {
    return this.core.fetch('/v1/networks/{network}/blockchains/{blockchainId}/transactions', 'get', metadata);
  }

  /**
   * Lists active staking transactions on the P-Chain for the supplied addresses.
   *
   * @summary List staking transactions
   */
  listActivePrimaryNetworkStakingTransactions(metadata: types.ListActivePrimaryNetworkStakingTransactionsMetadataParam): Promise<FetchResponse<200, types.ListActivePrimaryNetworkStakingTransactionsResponse200>> {
    return this.core.fetch('/v1/networks/{network}/blockchains/{blockchainId}/transactions:listStaking', 'get', metadata);
  }

  /**
   * Lists pending rewards on the Primary Network for the supplied addresses.
   *
   * @summary List pending rewards
   */
  listPendingPrimaryNetworkRewards(metadata: types.ListPendingPrimaryNetworkRewardsMetadataParam): Promise<FetchResponse<200, types.ListPendingPrimaryNetworkRewardsResponse200>> {
    return this.core.fetch('/v1/networks/{network}/rewards:listPending', 'get', metadata);
  }

  /**
   * Lists historical rewards on the Primary Network for the supplied addresses.
   *
   * @summary List historical rewards
   */
  listHistoricalPrimaryNetworkRewards(metadata: types.ListHistoricalPrimaryNetworkRewardsMetadataParam): Promise<FetchResponse<200, types.ListHistoricalPrimaryNetworkRewardsResponse200>> {
    return this.core.fetch('/v1/networks/{network}/rewards', 'get', metadata);
  }

  /**
   * Lists UTXOs on one of the Primary Network chains for the supplied addresses.
   *
   * @summary List UTXOs
   */
  getUtxosByAddresses(metadata: types.GetUtxosByAddressesMetadataParam): Promise<FetchResponse<200, types.GetUtxosByAddressesResponse200>> {
    return this.core.fetch('/v1/networks/{network}/blockchains/{blockchainId}/utxos', 'get', metadata);
  }

  /**
   * Gets primary network balances for one of the Primary Network chains for the supplied
   * addresses.
   *
   * C-Chain balances returned are only the shared atomic memory balance. For EVM balance,
   * use the `/v1/chains/:chainId/addresses/:addressId/balances:getNative` endpoint.
   *
   * @summary Get balances
   */
  getBalancesByAddresses(metadata: types.GetBalancesByAddressesMetadataParam): Promise<FetchResponse<200, types.GetBalancesByAddressesResponse200>> {
    return this.core.fetch('/v1/networks/{network}/blockchains/{blockchainId}/balances', 'get', metadata);
  }

  /**
   * Gets a block by block height or block hash on one of the Primary Network chains.
   *
   * @summary Get block
   */
  getBlockById(metadata: types.GetBlockByIdMetadataParam): Promise<FetchResponse<200, types.GetBlockByIdResponse200>> {
    return this.core.fetch('/v1/networks/{network}/blockchains/{blockchainId}/blocks/{blockId}', 'get', metadata);
  }

  /**
   * Lists the latest blocks proposed by a given NodeID on one of the Primary Network chains.
   *
   * @summary List blocks proposed by node
   */
  listPrimaryNetworkBlocksByNodeId(metadata: types.ListPrimaryNetworkBlocksByNodeIdMetadataParam): Promise<FetchResponse<200, types.ListPrimaryNetworkBlocksByNodeIdResponse200>> {
    return this.core.fetch('/v1/networks/{network}/blockchains/{blockchainId}/nodes/{nodeId}/blocks', 'get', metadata);
  }

  /**
   * Lists latest blocks on one of the Primary Network chains.
   *
   * @summary List latest blocks
   */
  listLatestPrimaryNetworkBlocks(metadata: types.ListLatestPrimaryNetworkBlocksMetadataParam): Promise<FetchResponse<200, types.ListLatestPrimaryNetworkBlocksResponse200>> {
    return this.core.fetch('/v1/networks/{network}/blockchains/{blockchainId}/blocks', 'get', metadata);
  }

  /**
   * Lists latest vertices on the X-Chain.
   *
   * @summary List vertices
   */
  listLatestXChainVertices(metadata: types.ListLatestXChainVerticesMetadataParam): Promise<FetchResponse<200, types.ListLatestXChainVerticesResponse200>> {
    return this.core.fetch('/v1/networks/{network}/blockchains/{blockchainId}/vertices', 'get', metadata);
  }

  /**
   * Gets a single vertex on the X-Chain.
   *
   * @summary Get vertex
   */
  getVertexByHash(metadata: types.GetVertexByHashMetadataParam): Promise<FetchResponse<200, types.GetVertexByHashResponse200>> {
    return this.core.fetch('/v1/networks/{network}/blockchains/{blockchainId}/vertices/{vertexHash}', 'get', metadata);
  }

  /**
   * Lists vertices at the given vertex height on the X-Chain.
   *
   * @summary List vertices by height
   */
  getVertexByHeight(metadata: types.GetVertexByHeightMetadataParam): Promise<FetchResponse<200, types.GetVertexByHeightResponse200>> {
    return this.core.fetch('/v1/networks/{network}/blockchains/{blockchainId}/vertices:listByHeight', 'get', metadata);
  }

  /**
   * Gets asset details corresponding to the given asset id on the X-Chain.
   *
   * @summary Get asset details
   */
  getAssetDetails(metadata: types.GetAssetDetailsMetadataParam): Promise<FetchResponse<200, types.GetAssetDetailsResponse200>> {
    return this.core.fetch('/v1/networks/{network}/blockchains/{blockchainId}/assets/{assetId}', 'get', metadata);
  }

  /**
   * Lists asset transactions corresponding to the given asset id on the X-Chain.
   *
   * @summary List asset transactions
   */
  listAssetTransactions(metadata: types.ListAssetTransactionsMetadataParam): Promise<FetchResponse<200, types.ListAssetTransactionsResponse200>> {
    return this.core.fetch('/v1/networks/{network}/blockchains/{blockchainId}/assets/{assetId}/transactions', 'get', metadata);
  }

  /**
   * Returns Primary Network chains that each address has touched in the form of an address
   * mapped array. If an address has had any on-chain interaction for a chain, that chain's
   * chain id will be returned.
   *
   * @summary Get chain interactions for addresses
   */
  getChainIdsForAddresses(metadata: types.GetChainIdsForAddressesMetadataParam): Promise<FetchResponse<200, types.GetChainIdsForAddressesResponse200>> {
    return this.core.fetch('/v1/networks/{network}/addresses:listChainIds', 'get', metadata);
  }

  /**
   * Gets network details such as validator and delegator stats.
   *
   * @summary Get network details
   */
  getNetworkDetails(metadata: types.GetNetworkDetailsMetadataParam): Promise<FetchResponse<200, types.GetNetworkDetailsResponse200>> {
    return this.core.fetch('/v1/networks/{network}', 'get', metadata);
  }

  /**
   * Lists all blockchains registered on the network.
   *
   * @summary List blockchains
   */
  listBlockchains(metadata: types.ListBlockchainsMetadataParam): Promise<FetchResponse<200, types.ListBlockchainsResponse200>> {
    return this.core.fetch('/v1/networks/{network}/blockchains', 'get', metadata);
  }

  /**
   * Lists all subnets registered on the network.
   *
   * @summary List subnets
   */
  listSubnets(metadata: types.ListSubnetsMetadataParam): Promise<FetchResponse<200, types.ListSubnetsResponse200>> {
    return this.core.fetch('/v1/networks/{network}/subnets', 'get', metadata);
  }

  /**
   * Lists details for validators. By default, returns details for all validators. Filterable
   * by validator node ids and minimum delegation capacity.
   *
   * @summary List validators
   */
  listValidators(metadata: types.ListValidatorsMetadataParam): Promise<FetchResponse<200, types.ListValidatorsResponse200>> {
    return this.core.fetch('/v1/networks/{network}/validators', 'get', metadata);
  }

  /**
   * List validator details for a single validator.  Filterable by validation status.
   *
   * @summary Get single validator details
   */
  getSingleValidatorDetails(metadata: types.GetSingleValidatorDetailsMetadataParam): Promise<FetchResponse<200, types.GetSingleValidatorDetailsResponse200>> {
    return this.core.fetch('/v1/networks/{network}/validators/{nodeId}', 'get', metadata);
  }

  /**
   * Lists details for delegators.
   *
   * @summary List delegators
   */
  listDelegators(metadata: types.ListDelegatorsMetadataParam): Promise<FetchResponse<200, types.ListDelegatorsResponse200>> {
    return this.core.fetch('/v1/networks/{network}/delegators', 'get', metadata);
  }

  /**
   * Create a new webhook.
   *
   * @summary Create a webhook
   */
  registerWebhook(body: types.RegisterWebhookBodyParam): Promise<FetchResponse<200, types.RegisterWebhookResponse200>> {
    return this.core.fetch('/v1/webhooks', 'post', body);
  }

  /**
   * Lists webhooks for the user.
   *
   * @summary List webhooks
   */
  listWebhooks(metadata?: types.ListWebhooksMetadataParam): Promise<FetchResponse<200, types.ListWebhooksResponse200>> {
    return this.core.fetch('/v1/webhooks', 'get', metadata);
  }

  /**
   * Retrieves a webhook by ID.
   *
   * @summary Get a webhook by ID
   */
  getWebhook(metadata: types.GetWebhookMetadataParam): Promise<FetchResponse<200, types.GetWebhookResponse200>> {
    return this.core.fetch('/v1/webhooks/{id}', 'get', metadata);
  }

  /**
   * Deactivates a webhook by ID.
   *
   * @summary Deactivate a webhook
   */
  deactivateWebhook(metadata: types.DeactivateWebhookMetadataParam): Promise<FetchResponse<200, types.DeactivateWebhookResponse200>> {
    return this.core.fetch('/v1/webhooks/{id}', 'delete', metadata);
  }

  /**
   * Updates an existing webhook.
   *
   * @summary Update a webhook
   */
  updateWebhook(body: types.UpdateWebhookBodyParam, metadata: types.UpdateWebhookMetadataParam): Promise<FetchResponse<200, types.UpdateWebhookResponse200>> {
    return this.core.fetch('/v1/webhooks/{id}', 'patch', body, metadata);
  }

  /**
   * Generates a new shared secret.
   *
   * @summary Generate a shared secret
   */
  generateSharedSecret(): Promise<FetchResponse<200, types.GenerateSharedSecretResponse200>> {
    return this.core.fetch('/v1/webhooks:generateOrRotateSharedSecret', 'post');
  }

  /**
   * Get a previously generated shared secret.
   *
   * @summary Get a shared secret
   */
  getSharedSecret(): Promise<FetchResponse<200, types.GetSharedSecretResponse200>> {
    return this.core.fetch('/v1/webhooks:getSharedSecret', 'get');
  }

  /**
   * Adding address(es) to a given webhook.
   *
   * @summary Add address(es) to a webhook
   */
  addAddressesToWebhook(body: types.AddAddressesToWebhookBodyParam, metadata: types.AddAddressesToWebhookMetadataParam): Promise<FetchResponse<200, types.AddAddressesToWebhookResponse200>> {
    return this.core.fetch('/v1/webhooks/{id}/addresses', 'patch', body, metadata);
  }

  /**
   * Removing address(es) from a given webhook.
   *
   * @summary Remove address(es) from a webhook
   */
  removeAddressesFromWebhook(body: types.RemoveAddressesFromWebhookBodyParam, metadata: types.RemoveAddressesFromWebhookMetadataParam): Promise<FetchResponse<200, types.RemoveAddressesFromWebhookResponse200>> {
    return this.core.fetch('/v1/webhooks/{id}/addresses', 'delete', body, metadata);
  }

  /**
   * Retrieving address(es) from a given webhook.
   *
   * @summary Get address(es) from a given webhook
   */
  getAddressesFromWebhook(metadata: types.GetAddressesFromWebhookMetadataParam): Promise<FetchResponse<200, types.GetAddressesFromWebhookResponse200>> {
    return this.core.fetch('/v1/webhooks/{id}/addresses', 'get', metadata);
  }

  /**
   * Gets a teleporter message by message ID.
   *
   * @summary Get a teleporter message
   */
  getTeleporterMessage(metadata: types.GetTeleporterMessageMetadataParam): Promise<FetchResponse<200, types.GetTeleporterMessageResponse200>> {
    return this.core.fetch('/v1/teleporter/messages/{messageId}', 'get', metadata);
  }

  /**
   * Lists teleporter messages. Ordered by timestamp in descending order.
   *
   * @summary List teleporter messages
   */
  listTeleporterMessages(metadata?: types.ListTeleporterMessagesMetadataParam): Promise<FetchResponse<200, types.ListTeleporterMessagesResponse200>> {
    return this.core.fetch('/v1/teleporter/messages', 'get', metadata);
  }

  /**
   * Gets native token balance of a wallet address.
   *
   * Balance at a given block can be retrieved with the `blockNumber` parameter.
   *
   * @summary Get native token balance
   */
  getNativeBalance(metadata: types.GetNativeBalanceMetadataParam): Promise<FetchResponse<200, types.GetNativeBalanceResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/addresses/{address}/balances:getNative', 'get', metadata);
  }

  /**
   * Lists ERC-20 token balances of a wallet address.
   *
   * Balance at a given block can be retrieved with the `blockNumber` parameter.
   *
   * Balance for specific contracts can be retrieved with the `contractAddresses` parameter.
   *
   * @summary List ERC-20 balances
   */
  listErc20Balances(metadata: types.ListErc20BalancesMetadataParam): Promise<FetchResponse<200, types.ListErc20BalancesResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/addresses/{address}/balances:listErc20', 'get', metadata);
  }

  /**
   * Lists ERC-721 token balances of a wallet address.
   *
   * Balance for a specific contract can be retrieved with the `contractAddress` parameter.
   *
   * @summary List ERC-721 balances
   */
  listErc721Balances(metadata: types.ListErc721BalancesMetadataParam): Promise<FetchResponse<200, types.ListErc721BalancesResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/addresses/{address}/balances:listErc721', 'get', metadata);
  }

  /**
   * Lists ERC-1155 token balances of a wallet address.
   *
   * Balance at a given block can be retrieved with the `blockNumber` parameter.
   *
   * Balance for a specific contract can be retrieved with the `contractAddress` parameter.
   *
   * @summary List ERC-1155 balances
   */
  listErc1155Balances(metadata: types.ListErc1155BalancesMetadataParam): Promise<FetchResponse<200, types.ListErc1155BalancesResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/addresses/{address}/balances:listErc1155', 'get', metadata);
  }

  /**
   * Lists ERC-721 and ERC-1155 token balances of a wallet address.
   *
   * Balance for a specific contract can be retrieved with the `contractAddress` parameter.
   *
   * @summary List collectible (ERC-721/ERC-1155) balances
   */
  listCollectibleBalances(metadata: types.ListCollectibleBalancesMetadataParam): Promise<FetchResponse<200, types.ListCollectibleBalancesResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/addresses/{address}/balances:listCollectibles', 'get', metadata);
  }

  /**
   * Lists the latest indexed blocks on the EVM-compatible chain sorted in descending order
   * by block timestamp.
   *
   * @summary List latest blocks
   */
  getLatestBlocks(metadata: types.GetLatestBlocksMetadataParam): Promise<FetchResponse<200, types.GetLatestBlocksResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/blocks', 'get', metadata);
  }

  /**
   * Gets the details of an individual block on the EVM-compatible chain.
   *
   * @summary Get block
   */
  getBlock(metadata: types.GetBlockMetadataParam): Promise<FetchResponse<200, types.GetBlockResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/blocks/{blockId}', 'get', metadata);
  }

  /**
   * If the address is a smart contract, returns the transaction in which it was deployed.
   *
   * @summary Get deployment transaction
   */
  getDeploymentTransaction(metadata: types.GetDeploymentTransactionMetadataParam): Promise<FetchResponse<200, types.GetDeploymentTransactionResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/contracts/{address}/transactions:getDeployment', 'get', metadata);
  }

  /**
   * Lists all contracts deployed by the given address.
   *
   * @summary List deployed contracts
   */
  listContractDeployments(metadata: types.ListContractDeploymentsMetadataParam): Promise<FetchResponse<200, types.ListContractDeploymentsResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/contracts/{address}/deployments', 'get', metadata);
  }

  /**
   * Gets metadata about the contract at the given address.
   *
   * @summary Get contract metadata
   */
  getContractMetadata(metadata: types.GetContractMetadataMetadataParam): Promise<FetchResponse<200, types.GetContractMetadataResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/addresses/{address}', 'get', metadata);
  }

  /**
   * Lists the supported EVM-compatible chains. Filterable by network.
   *
   * @summary List chains
   */
  supportedChains(metadata?: types.SupportedChainsMetadataParam): Promise<FetchResponse<200, types.SupportedChainsResponse200>> {
    return this.core.fetch('/v1/chains', 'get', metadata);
  }

  /**
   * Gets chain information for the EVM-compatible chain if supported by the api.
   *
   * @summary Get chain information
   */
  getChainInfo(metadata: types.GetChainInfoMetadataParam): Promise<FetchResponse<200, types.GetChainInfoResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}', 'get', metadata);
  }

  /**
   * Lists ERC transfers for an ERC-20, ERC-721, or ERC-1155 contract address.
   *
   * @summary List ERC transfers
   */
  listTransfers(metadata: types.ListTransfersMetadataParam): Promise<FetchResponse<200, types.ListTransfersResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/tokens/{address}/transfers', 'get', metadata);
  }

  /**
   * Returns a list of transactions where the given wallet address had an on-chain
   * interaction for the given chain. The ERC-20 transfers, ERC-721 transfers, ERC-1155, and
   * internal transactions returned are only those where the input address had an
   * interaction. Specifically, those lists only inlcude entries where the input address was
   * the sender (`from` field) or the receiver (`to` field) for the sub-transaction.
   * Therefore the transactions returned from this list may not be complete representations
   * of the on-chain data. For a complete view of a transaction use the
   * `/chains/:chainId/transactions/:txHash` endpoint.
   *
   * Filterable by block ranges.
   *
   * @summary List transactions
   */
  listTransactions(metadata: types.ListTransactionsMetadataParam): Promise<FetchResponse<200, types.ListTransactionsResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/addresses/{address}/transactions', 'get', metadata);
  }

  /**
   * Lists native transactions for an address. Filterable by block range.
   *
   * @summary List native transactions
   */
  listNativeTransactions(metadata: types.ListNativeTransactionsMetadataParam): Promise<FetchResponse<200, types.ListNativeTransactionsResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/addresses/{address}/transactions:listNative', 'get', metadata);
  }

  /**
   * Lists ERC-20 transfers for an address. Filterable by block range.
   *
   * @summary List ERC-20 transfers
   */
  listErc20Transactions(metadata: types.ListErc20TransactionsMetadataParam): Promise<FetchResponse<200, types.ListErc20TransactionsResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/addresses/{address}/transactions:listErc20', 'get', metadata);
  }

  /**
   * Lists ERC-721 transfers for an address. Filterable by block range.
   *
   * @summary List ERC-721 transfers
   */
  listErc721Transactions(metadata: types.ListErc721TransactionsMetadataParam): Promise<FetchResponse<200, types.ListErc721TransactionsResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/addresses/{address}/transactions:listErc721', 'get', metadata);
  }

  /**
   * Lists ERC-1155 transfers for an address. Filterable by block range.
   *
   * @summary List ERC-1155 transfers
   */
  listErc1155Transactions(metadata: types.ListErc1155TransactionsMetadataParam): Promise<FetchResponse<200, types.ListErc1155TransactionsResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/addresses/{address}/transactions:listErc1155', 'get', metadata);
  }

  /**
   * Returns a list of internal transactions for an address and chain. Filterable by block
   * range.
   *
   * Note that the internal transactions list only contains `CALL` or `CALLCODE` transactions
   * with a non-zero value and `CREATE`/`CREATE2` transactions. To get a complete list of
   * internal transactions use the `debug_` prefixed RPC methods on an archive node.
   *
   * @summary List internal transactions
   */
  listInternalTransactions(metadata: types.ListInternalTransactionsMetadataParam): Promise<FetchResponse<200, types.ListInternalTransactionsResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/addresses/{address}/transactions:listInternals', 'get', metadata);
  }

  /**
   * Gets the details of a single transaction.
   *
   * @summary Get transaction
   */
  getTransaction(metadata: types.GetTransactionMetadataParam): Promise<FetchResponse<200, types.GetTransactionResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/transactions/{txHash}', 'get', metadata);
  }

  /**
   * Lists the transactions that occured in a given block.
   *
   * @summary List transactions for a block
   */
  getTransactionsForBlock(metadata: types.GetTransactionsForBlockMetadataParam): Promise<FetchResponse<200, types.GetTransactionsForBlockResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/blocks/{blockId}/transactions', 'get', metadata);
  }

  /**
   * Lists the latest transactions. Filterable by status.
   *
   * @summary List latest transactions
   */
  listLatestTransactions(metadata: types.ListLatestTransactionsMetadataParam): Promise<FetchResponse<200, types.ListLatestTransactionsResponse200>> {
    return this.core.fetch('/v1/chains/{chainId}/transactions', 'get', metadata);
  }
}

const createSDK = (() => { return new SDK(); })()
;

export default createSDK;

export type { AddAddressesToWebhookBodyParam, AddAddressesToWebhookMetadataParam, AddAddressesToWebhookResponse200, DeactivateWebhookMetadataParam, DeactivateWebhookResponse200, GenerateSharedSecretResponse200, GetAddressesFromWebhookMetadataParam, GetAddressesFromWebhookResponse200, GetAssetDetailsMetadataParam, GetAssetDetailsResponse200, GetBalancesByAddressesMetadataParam, GetBalancesByAddressesResponse200, GetBlockByIdMetadataParam, GetBlockByIdResponse200, GetBlockMetadataParam, GetBlockResponse200, GetChainIdsForAddressesMetadataParam, GetChainIdsForAddressesResponse200, GetChainInfoMetadataParam, GetChainInfoResponse200, GetContractMetadataMetadataParam, GetContractMetadataResponse200, GetDeploymentTransactionMetadataParam, GetDeploymentTransactionResponse200, GetLatestBlocksMetadataParam, GetLatestBlocksResponse200, GetNativeBalanceMetadataParam, GetNativeBalanceResponse200, GetNetworkDetailsMetadataParam, GetNetworkDetailsResponse200, GetOperationResultMetadataParam, GetOperationResultResponse200, GetSharedSecretResponse200, GetSingleValidatorDetailsMetadataParam, GetSingleValidatorDetailsResponse200, GetTeleporterMessageMetadataParam, GetTeleporterMessageResponse200, GetTokenDetailsMetadataParam, GetTokenDetailsResponse200, GetTransactionMetadataParam, GetTransactionResponse200, GetTransactionsForBlockMetadataParam, GetTransactionsForBlockResponse200, GetTxByHashMetadataParam, GetTxByHashResponse200, GetUtxosByAddressesMetadataParam, GetUtxosByAddressesResponse200, GetVertexByHashMetadataParam, GetVertexByHashResponse200, GetVertexByHeightMetadataParam, GetVertexByHeightResponse200, GetWebhookMetadataParam, GetWebhookResponse200, HealthCheckResponse200, HealthCheckResponse503, ListActivePrimaryNetworkStakingTransactionsMetadataParam, ListActivePrimaryNetworkStakingTransactionsResponse200, ListAssetTransactionsMetadataParam, ListAssetTransactionsResponse200, ListBlockchainsMetadataParam, ListBlockchainsResponse200, ListCollectibleBalancesMetadataParam, ListCollectibleBalancesResponse200, ListContractDeploymentsMetadataParam, ListContractDeploymentsResponse200, ListDelegatorsMetadataParam, ListDelegatorsResponse200, ListErc1155BalancesMetadataParam, ListErc1155BalancesResponse200, ListErc1155TransactionsMetadataParam, ListErc1155TransactionsResponse200, ListErc20BalancesMetadataParam, ListErc20BalancesResponse200, ListErc20TransactionsMetadataParam, ListErc20TransactionsResponse200, ListErc721BalancesMetadataParam, ListErc721BalancesResponse200, ListErc721TransactionsMetadataParam, ListErc721TransactionsResponse200, ListHistoricalPrimaryNetworkRewardsMetadataParam, ListHistoricalPrimaryNetworkRewardsResponse200, ListInternalTransactionsMetadataParam, ListInternalTransactionsResponse200, ListLatestPrimaryNetworkBlocksMetadataParam, ListLatestPrimaryNetworkBlocksResponse200, ListLatestPrimaryNetworkTransactionsMetadataParam, ListLatestPrimaryNetworkTransactionsResponse200, ListLatestTransactionsMetadataParam, ListLatestTransactionsResponse200, ListLatestXChainVerticesMetadataParam, ListLatestXChainVerticesResponse200, ListNativeTransactionsMetadataParam, ListNativeTransactionsResponse200, ListPendingPrimaryNetworkRewardsMetadataParam, ListPendingPrimaryNetworkRewardsResponse200, ListPrimaryNetworkBlocksByNodeIdMetadataParam, ListPrimaryNetworkBlocksByNodeIdResponse200, ListSubnetsMetadataParam, ListSubnetsResponse200, ListTeleporterMessagesMetadataParam, ListTeleporterMessagesResponse200, ListTokensMetadataParam, ListTokensResponse200, ListTransactionsMetadataParam, ListTransactionsResponse200, ListTransfersMetadataParam, ListTransfersResponse200, ListValidatorsMetadataParam, ListValidatorsResponse200, ListWebhooksMetadataParam, ListWebhooksResponse200, PostTransactionExportJobBodyParam, PostTransactionExportJobResponse200, RegisterWebhookBodyParam, RegisterWebhookResponse200, ReindexNftMetadataParam, RemoveAddressesFromWebhookBodyParam, RemoveAddressesFromWebhookMetadataParam, RemoveAddressesFromWebhookResponse200, SupportedChainsMetadataParam, SupportedChainsResponse200, UpdateWebhookBodyParam, UpdateWebhookMetadataParam, UpdateWebhookResponse200 } from './types';
