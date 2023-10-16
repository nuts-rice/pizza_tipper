import { Button, Card, FormControl, FormLabel, Input, Stack } from '@chakra-ui/react'
import Image from 'next/image'
import Link from 'next/link'
import pizzaSend from 'public/icons/PizzaSendTip.png'
import { useEffect } from 'react'
import { toast } from 'react-hot-toast'
import { FC } from 'react'
import 'twin.macro'
import tw, { styled } from 'twin.macro'
export const ProfilePageTitle: FC = () => {
  const title = 'PizzaTipper profile page!'
  const desc = 'Give a pizza, post content.'
  return(
  <>
  Posts:
  <Input>
  </Input>
  Previous posts: 
  <div  />
</>
  )

}


