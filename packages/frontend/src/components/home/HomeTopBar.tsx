import Link from 'next/link'
import { FC } from 'react'
import { HiOutlineExternalLink } from 'react-icons/hi'
import 'twin.macro'

export const HomeTopBar: FC = () => {
  return (
    <>
      
        <div tw="font-bold">
          <span tw="hidden sm:inline">🍕Welcome to PizzaTipper home of on-chain pizza goodness!🍕</span>
          <span tw="inline sm:hidden">🍕🍕Pizza Tipper🍕🍕</span>
        </div>
      
    </>
  )
}
