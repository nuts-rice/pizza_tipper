import Image from 'next/image'
import Link from 'next/link'
import githubIcon from 'public/icons/github-button.svg'
import { FC } from 'react'
import 'twin.macro'
import tw, { styled } from 'twin.macro'
const StyledIconLink = styled(Link)(() => [
  tw`opacity-90 transition-all hover:(-translate-y-0.5 opacity-100)`,
])

export const FeedPageTitle: FC = () => {
  const title = 'Pizza tipper'
  const desc = 'Platform for tipping creators and cool people' 
  const githubHref = 'https://github.com/nuts_rice'
    return (
    <>
      <div tw="flex flex-col items-center text-center font-mono">
         <Link
          href={githubHref}
          target="_blank"
          className="group"
          tw="flex cursor-pointer items-center gap-4 rounded-3xl py-1.5 px-3.5 transition-all hover:bg-gray-900">
          <h1 tw="font-black text-[2.5rem]">{title}</h1>
        </Link>
      <p tw="mt4 mb-6 tesxt-gray-400">{desc}</p>
     <div tw="flex space-x-2">
     <StyledIconLink href={githubHref} target="_blank">
     <Image src ={githubIcon} priority height={32} alt="Github!" /> 
     </StyledIconLink>
    </div> 
    </div>
    </>
    )
}
