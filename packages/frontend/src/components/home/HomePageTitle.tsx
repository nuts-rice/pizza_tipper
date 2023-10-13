import Image from 'next/image'
import Link from 'next/link'
import inkathonLogo from 'public/brand/inkathon-logo.png'
import pizzaNewAccountLogo from 'public/icons/PizzaNewAccount.png'
import pizzaExistingAccountLogo from 'public/icons/PizzaExistingAccount.png' 
import githubIcon from 'public/icons/github-button.svg'
import sponsorIcon from 'public/icons/sponsor-button.svg'
import telegramIcon from 'public/icons/telegram-button.svg'
import vercelIcon from 'public/icons/vercel-button.svg'
import { FC } from 'react'
import 'twin.macro'
import tw, { styled } from 'twin.macro'

const StyledIconLink = styled(Link)(() => [
  tw`opacity-90 transition-all hover:(-translate-y-0.5 opacity-100)`,
])

export const HomePageTitle: FC = () => {
  const title = 'PizzaTipper!'
  const desc = 'Give a pizza, post content.'
  const githubHref = 'https://github.com/scio-labs/inkathon'
  const deployHref = 'https://github.com/scio-labs/inkathon#deployment'
  const sponsorHref = 'mailto:hello@scio.xyz'
  const telegramHref = 'https://t.me/inkathon'
  const newAccountHref = 'https://pizzatipper.xyz/new_profile'
  const existingAccountHref = 'https://pizzatipper.xyz/login'
  
  return (
    <>
      <div tw="flex flex-col items-center text-center font-mono">
        {/* Logo & Title */}
        <Link
          href={githubHref}
          target="_blank"
          className="group"
          tw="flex cursor-pointer items-center gap-4 rounded-3xl py-1.5 px-3.5 transition-all hover:bg-gray-900"
        >
          <Image src={inkathonLogo} priority width={60} alt="ink!athon Logo" />
          <h1 tw="font-black text-[2.5rem]">{title}</h1>
        </Link>
        
        {/* Tagline & Links */}
        <p tw="mt-2 text-gray-600 text-sm">
          By{' '}
          <a
            href="github.com/nuts_rice"
            target="_blank"
            tw="font-semibold text-gray-500 hover:text-gray-100"
          >
           Mishka Sistrunk 
          </a>{' '}
          </p>
        {/* Github & Vercel Buttons */}
        <div tw="flex space-x-2">
          <StyledIconLink href={newAccountHref} target="_blank">
            <Image src={pizzaNewAccountLogo} priority height={32} alt="New account" />
          </StyledIconLink> 
          <div>
          </div>
          
        </div>

        <div tw="my-14 w-14 bg-gray-800 h-[2px]" />
      </div>
    </>
  )
}
