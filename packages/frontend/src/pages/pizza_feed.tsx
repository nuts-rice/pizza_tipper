import {FeedTopBar } from '@components/feed/FeedTopBar'
import {FeedPageTitle} from '@components/feed/FeedPageTitle'
import { CenterBody } from '@components/layout/CenterBody'
import { ChainInfo } from '@components/web3/ChainInfo'
import { ConnectButton } from '@components/web3/ConnectButton'
import { useInkathon } from '@scio-labs/use-inkathon'

import type { NextPage } from 'next'
import { useEffect } from 'react'
import { toast } from 'react-hot-toast'
import 'twin.macro'


const FeedPage : NextPage = () => {
  const { error } = useInkathon()
  useEffect(() => {
    if (!error) return
    toast.error(error.message)
  }, [error])

  return (
  <>
    {}
    <FeedTopBar /> 
    <CenterBody tw = "mb-10 px-5">
    {}
    <FeedPageTitle/>
            {/* Connect Wallet Button */}
        <ConnectButton />

        <div tw="flex w-full flex-wrap items-start justify-center gap-4">
          {/* Chain Metadata Information */}
          <ChainInfo />


  </div>
  </CenterBody>
  </>
  )
}

export default FeedPage 




