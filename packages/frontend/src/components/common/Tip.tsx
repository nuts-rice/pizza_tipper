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
  const {contract, address: contractAddress} = useRegisteredContract(ContractIds.Tipper)
  const [tipState, setTipState] = useState<TIP_STATES>(TIP_STATES.ENTER_DETAILS)
  const [to, setTo] = useState<string>('')
  const [message, setMessage] = useState<string>('') ;
  const [pizzaCount, setPizzaCount] = useState<number>(1)
  const [tipId, setTipId] = useState<number>()
    // const alreadyExists = await useRegisteredContract(tipContract, tipId)
    const tipAmount =  useMemo(() => pizzaCount * tokenRate, [pizzaCount, tokenRate]);
    const handleTip = () => {
     onTip?.(to, tipAmount)
     }    
    setTipState(TIP_STATES.CONFIRMED)
     
  return(
  <></>
  )
  }
const EnterDataStates: FC<{
to: string
setTo: Dispatch<SetStateAction<string>>
message: string,
setMessage: Dispatch<SetStateAction<string>>
numberPizzas: number
setNumberPizzas: Dispatch<SetStateAction<number>>
submitTip: () => void 
}> = ({
to, 
setTo,
message,
setMessage,
numberPizzas,
setNumberPizzas
}) => {
  const { error } = useInkathon()
  useEffect(() => {
    if (!error) return
    toast.error(error.message)
  }, [error])
  //todo: should be event
  const validateAndSubmit = ()  => {
    // event.preventDefault()
// }
} 
return (
<form onSubmit= {validateAndSubmit}>
<FieldSet legend="">
<Input
label="Recipent Adress or AZERO.ID"/>
<Input
label="Tip message"/>
</FieldSet>

</form>
)
}
  // return (
  // <div>
  //     <label>
  //       To:
  //       <input 
  //         type="text" 
  //         placeholder="Recipient address or AZERO.ID" 
  //         value={to} 
  //         onChange={(e) => setTo(e.target.value)}
  //       />
  //     </label>

  //     <label>
  //       Message:
  //       <textarea 
  //         placeholder="Your message here..."
  //         value={message}
  //         onChange={(e) => setMessage(e.target.value)}
  //       />
  //     </label>

  //     <div>
  //       Pizza is {tokenRate} tokens
  //     </div>

  //     <label>
  //       How many pizzas
  //       <input 
  //         type="number" 
  //         min="1"
  //         value={pizzaCount}
  //         onChange={(e) => setPizzaCount(Number(e.target.value))}
  //       />
  //     </label>

  //     <button onClick={() => handleTip}>
  //       Send Tip
  //     </button>
  //   </div>
  // );

// }
export default Tip;
 
