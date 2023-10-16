import Image from 'next/image'
import Link from 'next/link'
import type { NextPage } from 'next'
import { useEffect } from 'react'
import { toast } from 'react-hot-toast'
import { FC,  useState } from 'react'
import 'twin.macro'
import tw, { styled } from 'twin.macro'


type State = {
  file?: File,
  name?: string,
  previewUrl?: string,
  type?: string
}

const initialState: State = {}
export const FileInput: FC = () => {
 return(<></>) 
}


