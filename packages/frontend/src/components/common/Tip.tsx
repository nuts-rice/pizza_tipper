import {Button, Card, Field, FieldSet,Input, Tag, Text} from 'degen'
import { SupportedChainId } from '@azns/resolver-core'
import { useResolveAddressToDomain, useResolveDomainToAddress } from '@azns/resolver-react'
import {Dispatch, FC , SetStateAction, useEffect, useMemo, useState} from 'react'
import 'twin.macro'
import { ContractIds } from '@deployments/deployments'
import tw, { styled } from 'twin.macro'
import { TipperContractInteractions } from '@components/web3/TipperContractInteractions'
import { useInkathon, useBalance, useRegisteredContract } from '@scio-labs/use-inkathon'
import {useSelector} from 'react-redux'
import { ConnectButton } from '@components/web3/ConnectButton'
import { ChainInfo } from '@components/web3/ChainInfo'
import { toast } from 'react-hot-toast'
import { contractTxWithToast } from '@utils/contractTxWithToast'
 
enum TIP_STATES {
  ENTER_DETAILS,
  REVIEW,
  CONFIRMED
}
interface TipProps {
  tokenRate?: number
  onTip?: (address: string, tipAmount: number)  => void;
}
//TODO: Grab tip rate from pizza oracle contract
const Tip: FC<TipProps> = ({tokenRate, onTip}: any)  => {
  const { error } = useInkathon()
  useEffect(() => {
    if (!error) return
    toast.error(error.message)
  }, [error])
  const {api, activeAccount, activeSigner} =  useInkathon()
  const [chainId, setChainId] = useState<SupportedChainId>(SupportedChainId.AlephZeroTestnet)
  const {contract, address: contractAddress} = useRegisteredContract(ContractIds.Tipper)
  const [tipState, setTipState] = useState<TIP_STATES>(TIP_STATES.ENTER_DETAILS)
  const [to, setTo] = useState<string>('')
  const [message, setMessage] = useState<string>('') ;
  const [pizzaCount, setPizzaCount] = useState<number>(1)
  const [tipId, setTipId] = useState<number>()
  const [customRouterAddress, setCustomRouterAddress] = useState<string>()
  const [lookupDomain, setLookupDomain] = useState<string>('domains.tzero')
  const [lookupAddress, setLookupAddress] = useState<string>(    '5EFJEY4DG2FnzcuCZpnRjjzT4x7heeEXuoYy1yAoUmshEhAP', )
    // const alreadyExists = await useRegisteredContract(tipContract, tipId)
  useEffect(() => {
    if (!error) return;
    toast.error(error.message);
  }, [error]);

  const tipAmount = useMemo(() => pizzaCount * (tokenRate || 0), [pizzaCount, tokenRate]);

  const domainResolver = useResolveDomainToAddress(lookupDomain, {
    debug: true,
    chainId,
    // Add additional properties as necessary
  });

  const handleTip = () => {
if (domainResolver.address && tipAmount) {
    onTip(domainResolver.address || '', tipAmount);
  }
    setTipState(TIP_STATES.CONFIRMED);
};
  return (
    <main>
      <form onSubmit={(e) => {
        e.preventDefault();
        handleTip();
      }}>
        <fieldset>
          <legend>Your Tip</legend>
          <Input
            label="Recipient Address or AZERO.ID"
            value={lookupDomain}
            onChange={(e) => setLookupDomain(e.target.value)}
          />
          {/* Display the result of domainResolver here */}
          <Input
            label="Tip Message"
            value={message}
            onChange={(e) => setMessage(e.target.value)}
            // Add necessary props to Input
          />
          <Input
            label="Number of pizzas"  
            min="1"
            value={pizzaCount}
            onChange={(e) => setPizzaCount(Number(e.target.value))}
            />
            </fieldset>
            </form>
</main>
)}
export default Tip;
 
