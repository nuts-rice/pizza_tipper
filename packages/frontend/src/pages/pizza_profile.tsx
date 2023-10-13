import Image from 'next/image'
import Link from 'next/link'
import  ProfilePageTitle  from '@components/profile/ProfilePageTitle'
import { FeedTopBar } from '@components/feed/FeedTopBar'
import { CenterBody } from '@components/layout/CenterBody'
import { TipperContractInteractions } from '@components/web3/TipperContractInteractions'
import pizzaSend from 'public/icons/PizzaSendTip.png'
import { useInkathon } from '@scio-labs/use-inkathon'
import type { NextPage } from 'next'
import { useEffect } from 'react'
import { toast } from 'react-hot-toast'
import 'twin.macro'
import tw, { styled } from 'twin.macro'

const StyledIconLink = styled(Link)(() => [
  tw`opacity-90 transition-all hover:(-translate-y-0.5 opacity-100)`,
])


const ProfilePage: NextPage = () => {
const { error } = useInkathon()
  useEffect(() => {
    if (!error) return
    toast.error(error.message)
  }, [error])

return (
<>
<CenterBody tw="mt-20 mb-10 px-5">
</CenterBody>
</>
)

}
