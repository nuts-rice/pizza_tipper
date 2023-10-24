import { env } from '@config/environment'
import { SubstrateDeployment } from '@scio-labs/use-inkathon'

export enum ContractIds {
  Greeter = 'greeter',
  Tipper = 'tipper',  
}

export const getDeployments = async (): Promise<SubstrateDeployment[]> => {
  const networks = env.supportedChains
  const deployments = networks
    .map(async (network) => [
      {
        contractId: ContractIds.Greeter,
        networkId: network,
        abi: await import(`@tipper/contracts/deployments/greeter/metadata.json`),
        address: (await import(`@tipper/contracts/deployments/greeter/${network}.ts`)).address,
      },
      {
        contractId: ContractIds.Greeter,
        networkId: network,
        // abi: await import('@')

      }
    ])
    .reduce(async (acc, curr) => [...(await acc), ...(await curr)], [] as any)

  return deployments
}
