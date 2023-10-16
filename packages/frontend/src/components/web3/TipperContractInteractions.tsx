import { Button, Card, FormControl, FormLabel, Input, Stack } from '@chakra-ui/react'
import { ContractIds } from '@deployments/deployments'
import {
  contractQuery,
  decodeOutput,
  useInkathon,
  useRegisteredContract,
} from '@scio-labs/use-inkathon'
import { contractTxWithToast } from '@utils/contractTxWithToast'
import { FC, useEffect, useState } from 'react'
import { useForm } from 'react-hook-form'
import toast from 'react-hot-toast'
import 'twin.macro'

type UpdateTipValues = {newTipMessage: string, newTipTo: string, newTipPizzas: number}
const useFetchTip = () => {
  const { api, activeAccount, activeSigner } = useInkathon()
  //FIX CONTRACT IDS
  const { contract, address: contractAddress } = useRegisteredContract(ContractIds.Tipper)

  const [tipMessage, setTipMessage] = useState<string>()
  const [tipTo, setTipTo] = useState<string>()
  const [tipPizzas, setTipPizzas] = useState<string>()


  const [fetchIsLoading, setFetchIsLoading] = useState<boolean>(false)
 const fetchTip = async () => {
    if (!api || !contract) return
    setFetchIsLoading(true)
    try {
      const result = await contractQuery(api, '', contract, 'tip')
      const { output, isError, decodedOutput } = decodeOutput(result, contract, 'tip')
      if (isError) throw new Error(decodedOutput)
      setTipMessage(output)
      setTipTo(output)
      setTipPizzas(output)
    } catch (e) {
      console.error(e)
      toast.error('Error while fetching tip. Try again...')
      setTipMessage(undefined)
      setTipTo(undefined)
      setTipPizzas(undefined)
    } finally {
      setFetchIsLoading(false)
    }
  }

  useEffect(() => {
    fetchTip()
  }, [contract])

  return { tipMessage, tipTo, tipPizzas, fetchIsLoading }
}

export const TipperContractInteractions: FC = () => {
  const { api, activeAccount, activeSigner } = useInkathon()
  //FIX CONTRACT IDS
  const { contract, address: contractAddress } = useRegisteredContract(ContractIds.Tipper)
  const [tipMessage, setTipMessage] = useState<string>()
  const [tipTo, setTipTo] = useState<string>()
  const [tipPizzas, setTipPizzas] = useState<string>()
  const [fetchIsLoading, setFetchIsLoading] = useState<boolean>()
  const [updateIsLoading, setUpdateIsLoading] = useState<boolean>()
  const { register, reset, handleSubmit } = useForm<UpdateTipValues>()
  
    const updateTip = async ({newTipMessage, newTipTo, newTipPizzas}:
    UpdateTipValues) => {
        if (!activeAccount || !contract || !activeSigner || !api) {
      toast.error('Wallet not connected. Try again…')
      return
    }
    setUpdateIsLoading(true) 
    try {
      await contractTxWithToast(api, activeAccount.address, contract, 'setTip', {}, [
        newTipMessage, newTipTo, newTipPizzas,
      ])
      reset()
    } catch (e) {
    console.error(e)

    } finally {
      setUpdateIsLoading(false)
      useFetchTip()
    }
  }
  if (!api) return null
    return (
    <>
    <div tw="flex grow flex-col space-y-4 max-w-[20rem]">
    <h2 tw="text-center font-mono text-gray-400">Tipper Smart Contract</h2>
    <Card variant ="outline" p={4} bgColor="whiteAlpha.100">
    <FormControl>
    <FormLabel> Tip </FormLabel>
    <Input
    // {/* TODO: load tip params */}
    placeholder={fetchIsLoading || !contract ? 'Loading': tipMessage }
    disabled={true}
    />
    </FormControl>
    </Card>
    <Card variant="outline" p={4} bgColor="whiteAlpha.100">
    <form onSubmit={handleSubmit(updateTip)}>
    <Stack direction="row" spacing={2} align="end">
    <FormControl>
    <FormLabel> Update tip message</FormLabel>
    <Input disabled={updateIsLoading} {...register('newTipMessage')}/>
    <FormLabel> Update tip destination</FormLabel>
    <Input disabled={updateIsLoading} {...register('newTipTo')}/>
    <FormLabel> Update amount of pizzas</FormLabel>
    <Input disabled={updateIsLoading} {...register('newTipPizzas')}/>
  </FormControl>
  <Button
    type="submit"
    mt={4}
    colorScheme="purple"
    isLoading={updateIsLoading}
    disabled={updateIsLoading}
    >
    Submit
    </Button>
    </Stack>
    </form>
    </Card>
    <p tw="text-center font-mono text-xs text-gray-600"> 
    {contract ? contractAddress: 'Loading...'}
    </p>
    </div>
    </>
    )
}
